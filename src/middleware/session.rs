use async_sqlx_session::PostgresSessionStore;
use std::time::Duration;
use tide::sessions::SessionMiddleware;
use tide::Result;

use domain::server_config::ServerConfig;

pub async fn init_session_middleware(
    config: &ServerConfig,
) -> Result<SessionMiddleware<PostgresSessionStore>> {

    // async sessions: https://docs.rs/async-session/latest/async_session/

    tide::log::info!("Connecting to postgres server ... for session storage");
    
    let store = PostgresSessionStore::new(&config.postgres_sql_connection_url).await?;

    store.spawn_cleanup_task(Duration::from_secs(60 * 60));
    store.migrate().await?;

    let session_hours_u64: u64 = config.session_ttl_hours.into();

    let middleware = tide::sessions::SessionMiddleware::new(
        store,
        String::from(&config.encryption_key_emoji).as_bytes(),
    )
    .with_cookie_name(String::from(&config.session_cookie_name))
    .with_session_ttl(Some(Duration::from_secs(session_hours_u64 * 60 * 60)));

    Ok(middleware)
}
