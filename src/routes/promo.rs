use tide::prelude::*;
use tide::{http::mime, Redirect, Request, Response, Result};

use crate::wiring::ServerWiring;

use askama::Template;

#[derive(Template)]
#[template(path = "promo.html.j2")]
struct PromoGetViewModel {}

pub async fn get(_req: Request<ServerWiring>) -> Result {
    let view_context = PromoGetViewModel {};

    let rendered_body = &view_context.render().unwrap();

    let response = Response::builder(200)
        .content_type(mime::HTML)
        .body_string(rendered_body.clone())
        .build();

    Ok(response)
}
