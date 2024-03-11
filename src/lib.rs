use std::io::Read;

use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;


/// A simple Spin HTTP component.
#[http_component]
async fn handle_hello_rust(req: Request) -> anyhow::Result<impl IntoResponse> {

   
        let data = r#"{"model": "mistral","prompt":"Here is a story about llamas eating grass"}"#;


        let req = Request::builder().method(Method::Post).uri("http://localhost:11434/api/generate").body(data).build();

        let response : Response = spin_sdk::http::send(req).await?; 
            Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response.into_body())
        .build())
            
}


