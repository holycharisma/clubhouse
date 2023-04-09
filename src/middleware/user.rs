use crate::wiring::ServerWiring;
use crate::util::encryption::SharedKeyring;
use domain::session::SessionUser;

#[derive(Default)]
pub struct UserExtensionMiddleware {}

impl UserExtensionMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

#[tide::utils::async_trait]
impl tide::Middleware<ServerWiring> for UserExtensionMiddleware {
    async fn handle(
        &self,
        mut req: tide::Request<ServerWiring>,
        next: tide::Next<'_, ServerWiring>,
    ) -> tide::Result {

        // if we find a registered user: 
        // - put the user in the request context
        // - sign an auth token for them 
        //   - give it back to the client in our response header

        let maybe_user: Option<SessionUser> = req.session().get("user");

        let auth_token = 
            if maybe_user.is_some() {
                let user = maybe_user.unwrap();
                let maybe_user = req.state().services.jwt_util.sign_auth_token(&user.email);
                req.set_ext(user);
                match maybe_user {
                    Ok(token) => Some(token),
                    _ => None
                }
            } else {
                None
            };

        if auth_token.is_some() {
            let secrets: &SharedKeyring = req.ext().unwrap();
            let encrypted = secrets.encrypt_broadcast_base64(&auth_token.unwrap()).await.unwrap();
            let mut res = next.run(req).await;
            res.insert_header("x-auth-token", encrypted.message);
            Ok(res)
        } else {
            Ok(next.run(req).await)
        }
    }
}