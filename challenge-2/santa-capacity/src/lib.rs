use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

use serde::{Deserialize, Serialize};
use serde_json::{from_slice, Value};

#[derive(serde::Deserialize)]
struct SantaData {
    kids: Vec<i32>,
    weight: Vec<i32>,
    capacity: i32,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_santa_capacity(req: Request) -> anyhow::Result<impl IntoResponse> {
    let santa_data: SantaData = serde_json::from_slice(req.body())?;

    let mut ordered = santa_data
        .kids
        .iter()
        .zip(santa_data.weight.iter())
        .collect::<Vec<_>>();

    ordered.sort_by_key(|&(num, _)| -num);

    println!("{:?}", ordered);

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")?)
}
