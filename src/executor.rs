use std::time::Instant;


use crate::requests::model::{HttpMethod, Request, Response};


pub async fn execute_request(request: &Request) -> Response {
    let start = Instant::now();
    let response = match request.method {
        HttpMethod::Get => reqwest::get(request.url.clone()).await.unwrap(),
        _ => todo!("Not implemented")
    };

    let duration = start.elapsed();

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await.unwrap();

    Response {
        status,
        duration,
        headers,
        body
    }

    
    
}
