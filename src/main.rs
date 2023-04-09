mod dao;
mod middleware;
mod routes;
mod util;
mod wiring;

use wiring::ServerWiring;

#[macro_use]
extern crate lazy_static;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let config = ServerWiring::init_server_config();

    // probably need to validate the config...
    // is the session secret legit byte key?
    // is the postgres url reachable?
    // are the jwt signing keys valid?
    let server_wiring = ServerWiring::new(&config).await?;

    dao::user::UserDao::insert_super_user(&config, &server_wiring)
        .await
        .unwrap();

    let mut app = tide::with_state(server_wiring);

    let session_middleware = middleware::session::init_session_middleware(&config).await?;
    let anti_forgery_middleware = crate::middleware::security::AntiRequestForgeryMiddleware::new();
    let keyring_middleware = crate::middleware::keyring::SessionEncryptionMiddleware::new();
    let user_ext_middleware = middleware::user::UserExtensionMiddleware::new();

    let user_authorization_middleware =
        middleware::authorization::UserAuthorizationMiddleware::new();

    // these global middlewares run on every request...
    app.with(session_middleware);
    app.with(keyring_middleware);
    app.with(anti_forgery_middleware);
    app.with(user_ext_middleware);

    app.at("/").get(routes::index::get);

    // https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html
    app.at("/hcc_frame.js").get(routes::hcc_frame_js::get);

    app.at("/handshake").get(routes::handshake::get);

    app.at("/login")
        .get(routes::user::login::get)
        .post(routes::user::login::post);

    app.at("/signup")
        .get(routes::user::signup::get)
        .post(routes::user::signup::post);

    app.at("/media").get(routes::media::list::get);

    app.at("/promo").get(routes::promo::get);

    app.at("/header").get(routes::brand::get_header);
    app.at("/splash").get(routes::brand::get_splash);
    app.at("/sidebar").get(routes::brand::get_sidebar);
    app.at("/footer").get(routes::brand::get_footer);

    app.at("/app").get(routes::app::get);

    app.at("/disconnect").post(routes::disconnect::post);

    app.at("/api/secret")
        .with(user_authorization_middleware)
        .get(routes::dummy_secret::get);

    // TODO: all these assets shouldn't be served by this server....
    // they should probably be through some kind of CDN or somethin

    // TODO: would be nice if this was a little smarter
    let hcc_client_dist = config.client_dist_dir;

    // "/home/orpheus/src/hcc/hcc-client/dist/# ";
    let favicon_path = format!("{}{}", hcc_client_dist, "favicon.svg");

    app.at("/hcc/*")
        .serve_dir(hcc_client_dist)
        .expect("Failed to load frontend assets");

    app.at("/favicon.svg")
        .serve_file(favicon_path)
        .expect("No favicon found");

    app.listen(&config.bind_url).await?;

    Ok(())
}
