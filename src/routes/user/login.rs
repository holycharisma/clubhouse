use tide::prelude::*;
use tide::{http::mime, Redirect, Request, Response, Result};

use crate::dao;
use crate::util::emoji;
use crate::util::encryption;
use crate::util::password::PasswordUtil;
use crate::wiring::ServerWiring;
use domain::session::SessionUser;

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "user/login.html.j2")] // using the template in this path, relative
struct LoginGetView {}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let maybe_user: Option<&SessionUser> = req.ext();

    if maybe_user.is_some() {
        Ok(Redirect::new("/app").into())
    } else {
        let login_get_view = LoginGetView {};
        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let encrypted_body = secrets
            .encrypt_broadcast_emoji(&login_get_view.render().unwrap())
            .await
            .unwrap()
            .message;

        let response = Response::builder(200)
            .content_type(mime::HTML)
            .body_string(encrypted_body)
            .build();
        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
struct UserLoginDto {
    email: String, // emoji encrypted fields
    password: String,
}

pub async fn post(mut req: Request<ServerWiring>) -> Result {
    let form = {
        let encrypted_form: UserLoginDto = req.body_form().await?;

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let sender = &secrets.user;

        let encrypted_email = encryption::UserEncryptedEmojiMessage {
            sender: sender.to_owned(),
            message: encrypted_form.email,
        };

        let encrypted_password = encryption::UserEncryptedEmojiMessage {
            sender: sender.to_owned(),
            message: encrypted_form.password,
        };

        UserLoginDto {
            email: encrypted_email.decrypt(secrets).unwrap(),
            password: encrypted_password.decrypt(secrets).unwrap(),
        }
    };

    let plaintext_email = &form.email.as_bytes();
    let wiring: &ServerWiring = &req.state();
    let search = dao::user::UserDao::find_by_email(wiring, plaintext_email)
        .await
        .unwrap();

    if search.is_none() {
        let response = Response::builder(403).build();
        Ok(response)
    } else {
        let u = search.unwrap();

        let user_pwhash = emoji::decode(&u.password);
        let expected_email_hash = u.email_hash;

        let form_email_hash = {
            encryption::DeterministicEmojiEncrypt::new(
                &req.state().config.encryption_key_emoji,
                &req.state().config.encryption_salt_emoji,
                plaintext_email.to_owned(),
            )
        }
        .unwrap();

        let email_is_valid = form_email_hash.encrypted == expected_email_hash;

        let pass_is_valid =
            email_is_valid && { PasswordUtil::verify_hashed_bytes(&form.password, &user_pwhash) };

        if email_is_valid && pass_is_valid {
            let super_email = &wiring.config.super_user_email.as_bytes();

            let is_admin_user = plaintext_email == super_email;

            let session = req.session_mut();

            let user = SessionUser {
                email: String::from(&form.email),
                is_admin: is_admin_user,
            };

            let _res = session.insert("user", user.clone()).unwrap();

            // redirect to app now that we have set user
            Ok(Redirect::new("/app").into())
        } else {
            tide::log::info!("Failed login for user: {}", form.email);
            let response = Response::builder(403).build();
            Ok(response)
        }
    }
}
