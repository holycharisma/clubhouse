use crate::wiring::ServerWiring;
use crate::util::encryption::{SharedKeyring, EncryptedKeyring};

#[derive(Default)]
pub struct SessionEncryptionMiddleware {

}

impl SessionEncryptionMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

#[tide::utils::async_trait]
impl tide::Middleware<ServerWiring> for SessionEncryptionMiddleware {
    async fn handle(
        &self,
        mut req: tide::Request<ServerWiring>,
        next: tide::Next<'_, ServerWiring>,
    ) -> tide::Result {
        let s = req.session();
        let secrets = match s.get::<EncryptedKeyring>("keyring") {
            Some(secrets) => {
                secrets.open(&req.state().config).expect("decrypted keyring")
            },
            None => {
                let secrets = SharedKeyring::new().await.unwrap();
                let config = req.state().config.clone();
                let m = req.session_mut();
                let e = EncryptedKeyring::seal(&secrets, &config).expect("encrypted keyring");
                m.insert("keyring", e).expect("serializable");
                secrets
            }
        };
        req.set_ext(secrets);
        Ok(next.run(req).await)
    }
}
