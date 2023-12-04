use http::{Method, StatusCode};
use spin_sdk::{
    http::{IntoResponse, Response},
    http_component,
    key_value::Store,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_wish_list(req: http::Request<Vec<u8>>) -> anyhow::Result<impl IntoResponse> {
    let store = Store::open_default()?;

    let (status, body) = match *req.method() {
        Method::POST => {
            store.set(req.uri().path(), req.body().as_slice())?;
            println!(
                "Storing value in the KV store with {:?} as the key",
                req.uri().path()
            );
            (StatusCode::CREATED, None)
        }
        Method::GET => {
            // Get the value associated with the request URI, or return a 404 if it's not present
            match store.get(req.uri().path())? {
                Some(value) => {
                    println!("Found value for the key {:?}", req.uri().path());
                    (StatusCode::OK, Some(value))
                }
                None => {
                    println!("No value found for the key {:?}", req.uri().path());
                    (StatusCode::NOT_FOUND, None)
                }
            }
        }
        _ => (StatusCode::NOT_FOUND, None),
    };
    //Ok(Response::new(status, body))
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}
