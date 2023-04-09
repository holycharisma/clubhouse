use tide::prelude::*;
use tide::{http::mime, Redirect, Request, Response, Result};

use crate::dao;
use crate::util::emoji;
use crate::util::encryption;
use crate::util::password::PasswordUtil;
use crate::wiring::ServerWiring;

use domain::session::SessionUser;

use askama::Template;

#[derive(Template)]
#[template(path = "brand/header.html.j2")]
struct BrandHeaderViewModel {}

#[derive(Template)]
#[template(path = "brand/sidebar.html.j2")]
struct BrandSidebarViewModel {}

#[derive(Template)]
#[template(path = "brand/splash.html.j2")]
struct BrandSplashViewModel {}

#[derive(Template)]
#[template(path = "brand/footer.html.j2")]
struct BrandFooterViewModel {}

pub async fn get_header(req: Request<ServerWiring>) -> Result {
    let view_context = BrandHeaderViewModel {};

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

pub async fn get_sidebar(req: Request<ServerWiring>) -> Result {
    let view_context = BrandSidebarViewModel {};

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

pub async fn get_splash(req: Request<ServerWiring>) -> Result {
    let view_context = BrandSplashViewModel {};

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

pub async fn get_footer(req: Request<ServerWiring>) -> Result {
    let view_context = BrandFooterViewModel {};

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
