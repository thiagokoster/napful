use std::time::Instant;

use reqwest::header;

use crate::requests::model::{HttpMethod, Request, Response};

pub async fn execute_request(request: &Request) -> Result<Response, reqwest::Error> {
    let start = Instant::now();
    let client = reqwest::Client::new();
    let mut http_request = match request.method {
        HttpMethod::Get => client.get(request.url.clone()),
        HttpMethod::Post => client.post(request.url.clone()),
        _ => todo!("Not implemented"),
    };

    // Add a body if applicable
    if let Some(body) = &request.body {
        http_request = http_request.body(body.clone());
    }

    http_request = http_request.header(header::CONTENT_TYPE, "application/json");

    let response = http_request.send().await?;

    let duration = start.elapsed();

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await.unwrap();

    Ok(Response {
        status,
        duration,
        headers,
        body,
    })
}
