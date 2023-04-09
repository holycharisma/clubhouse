use crate::wiring::ServerWiring;
use crate::util::encryption::{SharedKeyring, UserEncryptedBase64Message};
use domain::session::SessionUser;

#[derive(Default)]
pub struct UserAuthorizationMiddleware {}

impl UserAuthorizationMiddleware {
    pub fn new() -> Self {
        Self {}
    }

    fn unauthorized() -> tide::Result<tide::Response> {
        Ok(tide::Response::builder(403).build())
    }
}

#[tide::utils::async_trait]
impl tide::Middleware<ServerWiring> for UserAuthorizationMiddleware {
    async fn handle(
        &self,
        req: tide::Request<ServerWiring>,
        next: tide::Next<'_, ServerWiring>,
    ) -> tide::Result {
        let maybe_user: Option<&SessionUser> = req.ext();
        if maybe_user.is_some() {
            let user = maybe_user.unwrap();
            let maybe_header = req.header("x-auth-token");
            if maybe_header.is_some() {
                let maybe_token_text = maybe_header.unwrap().into_iter().next();
                if maybe_token_text.is_some() {
                    let jwt_util = &req.state().services.jwt_util;
                    let secrets: &SharedKeyring = req.ext().unwrap();

                    let message = UserEncryptedBase64Message {
                        message: maybe_token_text.unwrap().as_str().to_owned()
                    };

                    let decrypted = message.decrypt(secrets).unwrap();

                    let verification = jwt_util.verify_auth_token(&decrypted, &user.email);
                    if verification.is_ok() {
                        Ok(next.run(req).await)
                    } else {
                        tide::log::info!("Invalid authorization token");
                        UserAuthorizationMiddleware::unauthorized()
                    }
                } else {
                    tide::log::info!("Missing authorization token");
                    UserAuthorizationMiddleware::unauthorized()
                }
            } else {
                tide::log::info!("Missing authorization token");
                UserAuthorizationMiddleware::unauthorized()
            }
        } else {
            tide::log::info!("Missing required session user");
            UserAuthorizationMiddleware::unauthorized()
        }

    }
}