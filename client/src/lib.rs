use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use std::collections::HashMap;

// A strucut to hold the generateData
#[derive(Debug, Serialize, Deserialize)]
pub struct Responses {
    all: HashMap<String, Vec<String>>
}

// A strucut to hold the calculated data
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingResults {
    distribution: HashMap<String, HashMap<String, i32>>,
    most_common: HashMap<String, String>,
    average: HashMap<String, f32>
}

#[wasm_bindgen]
pub fn run() -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "http://localhost:3000",
        &opts,
    )
    .unwrap();

    request
        .headers()
        .set("Accept", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            let _raw: Responses = json.into_serde().unwrap();

            let mut _dates_distribution: HashMap<String, HashMap<String, i32>> = HashMap::new();

            let mut _dates_most_common: HashMap<String, String> = HashMap::new();

            let mut _dates_averages: HashMap<String, f32> = HashMap::new();

            // iterate over everything.
            for (date, responses) in &_raw.all {
                let mut _most_common = "0".to_string();
                let count:usize = responses.iter().count();
                let mut sum:f32 = 0.00;
                let mut temp = 0;
                let mut distribution: HashMap<String, i32> = HashMap::new();
                for response in responses {
                    if !&distribution.contains_key(&response.to_string()) {
                        &distribution.insert(response.to_string(), 1);
                    }
                    else{
                        *distribution.get_mut(&response.to_string()).unwrap() += 1;
                    }

                    if *distribution.get_mut(&response.to_string()).unwrap() > temp {
                        temp = *distribution.get_mut(&response.to_string()).unwrap();
                        _most_common = response.to_string();
                    }
                    sum = sum + &response.to_string().parse::<f32>().unwrap();
                }

                _dates_averages.insert(date.to_string(), sum / count.to_string().parse::<f32>().unwrap());
                _dates_most_common.insert(date.to_string(), _most_common);
                _dates_distribution.insert(date.to_string(), distribution);
            }

            let reporting_result: ReportingResults = ReportingResults{
                distribution: _dates_distribution,
                most_common: _dates_most_common,
                average: _dates_averages
            };

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(JsValue::from_serde(&reporting_result).unwrap())
        });

    // Convert this Rust `Future` back into a JS `Promise`.
    future_to_promise(future)
}
