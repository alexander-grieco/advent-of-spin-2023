use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

use serde::Deserialize;
use serde_json::{from_slice, json, to_string};

#[derive(Deserialize)]
struct SantaData {
    kids: Vec<i32>,
    weight: Vec<i32>,
    capacity: i32,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_santa_capacity(req: Request) -> anyhow::Result<impl IntoResponse> {
    let santa_data: SantaData = from_slice(req.body())?;

    let mut ordered = santa_data
        .kids
        .iter()
        .zip(santa_data.weight.iter())
        .collect::<Vec<_>>();

    ordered.sort_by_key(|&(num, _)| -num);

    let num_kids = ordered
        .iter()
        .scan(0, |state, (num, weight)| {
            *state += **weight as i32;
            if *state > santa_data.capacity {
                return None;
            }
            Some((num, *state))
        })
        .map(|(num, _)| *num)
        .sum::<i32>();

    let json_obj = to_string(&json!({
        "kids": num_kids
    }))
    .unwrap();

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_obj)?)
}
