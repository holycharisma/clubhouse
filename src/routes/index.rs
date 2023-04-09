
use tide::{http::mime, Request, Response, Result};
use crate::wiring::ServerWiring;

// for now - maybe forever:
// just serve the relative dist folder index as an iframe from the rs-wasm sibling project

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "index.html.j2")] // using the template in this path, relative
struct IndexView {}

pub async fn get(_req: Request<ServerWiring>) -> Result {

    let view = IndexView {};

    let response_body = view.render().unwrap();

    let response = Response::builder(200)
        .content_type(mime::HTML)
        .body_string(response_body)
        .build();

    Ok(response)
}