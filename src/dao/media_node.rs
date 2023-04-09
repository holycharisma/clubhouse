// use crate::util::encryption;
// use crate::wiring::ServerWiring;
// use domain::server_config::ServerConfig;

use domain::sea_orm::entities::prelude::MediaNode;
use domain::sea_orm::entities::user_email_password;

use sea_orm::*;

pub struct MediaNodeDao {}

impl MediaNodeDao {
    /*

    pub async fn find_by_email(
        wiring: &ServerWiring,
        email_plaintext_bytes: &[u8],
    ) -> Result<Option<user_email_password::Model>, ()> {
        let hash = encryption::get_masked_hash(
            &wiring.config.encryption_key_emoji,
            &wiring.config.encryption_salt_emoji,
            email_plaintext_bytes,
        )
        .unwrap();

        let matches_hash = user_email_password::Column::EmailHash.eq(hash);

        let res = UserEmailPassword::find()
            .filter(matches_hash)
            .limit(1)
            .one(&wiring.db)
            .await;

        if res.is_ok() {
            let i = res.unwrap();
            Ok(i)
        } else {
            Err(())
        }
    }

    pub async fn insert_super_user(config: &ServerConfig, wiring: &ServerWiring) -> Result<(), ()> {
        let plaintext_login = &config.super_user_email.as_bytes();

        let already_exists = Self::find_by_email(wiring, plaintext_login).await.unwrap();

        if already_exists.is_some() {
            tide::log::info!("super user already exists!");
        } else {
            tide::log::info!("super user does not exist!");

            let em = encryption::DeterministicEmojiEncrypt::new(
                &config.encryption_key_emoji,
                &config.encryption_salt_emoji,
                plaintext_login,
            )
            .unwrap();

            let encrypted_email = String::from(em.encrypted.clone());
            let email_hash = String::from(em.hash.clone());
            let encoded_hash = String::from(config.super_user_pwhash_emoji.clone());

            let s = user_email_password::ActiveModel {
                email: Set(encrypted_email),
                email_hash: Set(email_hash),
                password: Set(encoded_hash),
                active: Set(true),
                ..Default::default()
            };

            let operation = UserEmailPassword::insert(s).exec(&wiring.db).await;

            if operation.is_ok() {
                let item = operation.ok();
                println!("INSERTED ONE: {:?}", item);
            } else {
                println!(
                    "Failed to insert super user... maybe it already exists?? {:?}",
                    operation.err()
                );
            }
        }

        Ok(())
    }

    */
}
