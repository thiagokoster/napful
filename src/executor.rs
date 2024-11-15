use std::time::Instant;

use serde_json::Value;

use crate::requests::model::{HttpMethod, Request, Response};

pub async fn execute_request(request: &Request, formatted: bool) -> Result<Response, reqwest::Error> {
    let start = Instant::now();
    let client = reqwest::Client::new();
    let mut http_request = match request.method {
        HttpMethod::Get => client.get(request.url.clone()),
        HttpMethod::Post => client.post(request.url.clone()),
        HttpMethod::Put => client.put(request.url.clone()),
        HttpMethod::Delete => client.delete(request.url.clone()),
        _ => todo!("Not implemented"),
    };

    // Add a body if applicable
    if let Some(body) = &request.body {
        http_request = http_request.body(body.clone());
    }

    for (key, value) in request.headers.iter() {
        http_request = http_request.header(key, value);
    }


    let response = http_request.send().await?;

    let duration = start.elapsed();

    let status = response.status();
    let headers = response.headers().clone();
    let body = if formatted {
        let response_body: Value = response.json().await.unwrap();
        serde_json::to_string_pretty(&response_body).unwrap()
    } else{
        response.text().await.unwrap()
    };

    Ok(Response {
        status,
        duration,
        headers,
        body,
    })
}
