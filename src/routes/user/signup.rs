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
#[template(path = "user/signup.html.j2")] // using the template in this path, relative
struct SignupGetViewModel {}

pub async fn get(req: Request<ServerWiring>) -> Result {
    let maybe_user: Option<&SessionUser> = req.ext();

    if maybe_user.is_some() {
        Ok(Redirect::new("/app").into())
    } else {
        let view_context = SignupGetViewModel {};

        let secrets: &encryption::SharedKeyring = req.ext().unwrap();

        let encrypted_body = secrets
            .encrypt_broadcast_emoji(&view_context.render().unwrap())
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
struct UserSignupDto {
    email: String, // emoji encrypted fields
    password_bcrypt: String,
}

pub async fn post(mut req: Request<ServerWiring>) -> Result {
    return Ok(Redirect::new("/").into());
    /*
    let form = {
        let encrypted_form: UserSignupDto = req.body_form().await?;

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

    let search = dao::user::UserDao::find_by_email(&req.state(), &form.email.as_bytes())
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
            encryption::DeterministicEmojiEncrypted::new(
                &req.state().config.encryption_key_emoji,
                &req.state().config.encryption_salt_emoji,
                form.email.as_bytes()
            )
        }.unwrap();

        let email_is_valid = form_email_hash == expected_email_hash;

        let pass_is_valid = email_is_valid && {
            PasswordUtil::verify_hashed_bytes(&form.password, &user_pwhash)
        };

        if email_is_valid && pass_is_valid {
            let session = req.session_mut();

            let user = SessionUser {
                email: String::from(&form.email),
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

        */
}
