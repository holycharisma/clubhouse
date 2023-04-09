use tide::{Request, Response, Result};

use crate::wiring::ServerWiring;


pub async fn post(mut req: Request<ServerWiring>) -> Result {
    let session = req.session_mut();

    session.destroy();

    Ok(Response::builder(200)
        .header("HX-Redirect", "/hcc/disconnect.html")
        .build())
}
