use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use crate::util::jwt::{JsonWebTokenSecrets, JsonWebTokenUtil};
use domain::server_config::ServerConfig;

#[derive(Clone)]
pub struct ServerWiring {
    pub services: ServiceWiring,
    pub db: DatabaseConnection,
    pub config: ServerConfig,
}

impl ServerWiring {
    pub async fn database(config: &ServerConfig) -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(&config.postgres_sql_connection_url).await;
        db
    }

    pub fn init_server_config() -> ServerConfig {
        dotenv().ok();
        ServerConfig {
            domain: env::var("HCC_ORIGIN_DOMAIN")
                .expect("Invalid configuration: HCC_ORIGIN_DOMAIN required"),
            session_cookie_name: env::var("HCC_SESSION_COOKIE_NAME")
                .expect("Invalid configuration: HCC_SESSION_COOKIE_NAME required"),
            session_ttl_hours: env::var("HCC_SESSION_TTL_HOURS")
                .expect("Invalid configuration: HCC_SESSION_TTL_HOURS required")
                .parse()
                .expect("Invalid configuration: HCC_SESSION_TTL_HOURS must be a number"),
            encryption_key_emoji: env::var("HCC_ENCRYPTION_KEY_EMOJI")
                .expect("Invalid configuration: HCC_ENCRYPTION_KEY_EMOJI required"),
            encryption_salt_emoji: env::var("HCC_ENCRYPTION_SALT_EMOJI")
                .expect("Invalid configuration: HCC_ENCRYPTION_SALT_EMOJI required"),
            rsa_private_key_path: env::var("HCC_RSA_PRIVATE_KEY_PATH")
                .expect("Invalid configuration: HCC_RSA_PRIVATE_KEY_PATH required"),
            rsa_public_key_path: env::var("HCC_RSA_PUBLIC_KEY_PATH")
                .expect("Invalid configuration: HCC_RSA_PUBLIC_KEY_PATH required"),
            postgres_sql_connection_url: env::var("HCC_POSTGRES_SQL_CONNECTION_URL")
                .expect("Invalid configuration: HCC_POSTGRES_SQL_CONNECTION_URL required"),
            bind_url: env::var("HCC_BIND_URL")
                .expect("Invalid configuration: HCC_BIND_URL required"),
            super_user_email: env::var("HCC_SUPER_USER_EMAIL")
                .expect("Invalid configuration: HCC_SUPER_USER_EMAIL required"),
            super_user_pwhash_emoji: env::var("HCC_SUPER_USER_PWHASH_EMOJI")
                .expect("Invalid configuration: HCC_SUPER_USER_PWHASH_EMOJI required"),
            client_dist_dir: env::var("HCC_CLIENT_DIST_DIR")
                .expect("Invalid configuration: HCC_CLIENT_DIST_DIR required"),
        }
    }

    pub async fn new(server_config: &ServerConfig) -> Result<ServerWiring, tide::Error> {
        let config = server_config.to_owned();
        let server_state = ServerWiring {
            services: ServiceWiring {
                jwt_util: Arc::new(ServiceWiring::jwt_util(&config)),
            },
            db: {
                tide::log::info!("Trying to connect to sea-orm db...");
                let db = ServerWiring::database(&config).await?;
                tide::log::info!("sea-orm connect: OK!");
                db
            },
            config: server_config.to_owned(),
        };
        Ok(server_state)
    }
}

#[derive(Clone)]
pub struct ServiceWiring {
    pub jwt_util: Arc<JsonWebTokenUtil>,
}

impl ServiceWiring {
    pub fn jwt_util(config: &ServerConfig) -> JsonWebTokenUtil {
        let rsa_secrets = JsonWebTokenSecrets::read_keys(
            &config.rsa_private_key_path,
            &config.rsa_public_key_path,
        );

        JsonWebTokenUtil {
            secrets: rsa_secrets,
            issuer: String::from(&config.domain),
            expiry_duration_millis: (config.session_ttl_hours * 1000 * 60 * 60) as i64,
        }
    }
}
