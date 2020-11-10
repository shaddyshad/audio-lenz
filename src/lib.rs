extern crate hyper;
extern crate hyper_rustls;

use hyper::{Client, Body, Method, Request, body, Error};
use hyper_rustls::HttpsConnector;
use tokio::runtime::Runtime;


pub fn get_access_tokens() -> Result<body::Bytes, Error>{
    // build a https instance
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    // azure specific details 
    let url = "https://eastus.api.cognitive.microsoft.com/sts/v1.0/issuetoken";
    let key = "4a1c4999aff742b6889c5b1a5bfdbfe0";

    // build a post request 
    let req = Request::builder()
                .uri(url)
                .method(Method::POST)
                .header("Ocp-Apim-Subscription-Key", key)
                .header("Content-Length", "0")
                .header("Content-Type","application/x-www-form-urlencoded")
                .body(Body::default())
                .unwrap();

    // create a tokio runtime 
    let mut rt = Runtime::new().unwrap();

    let resp = rt.block_on(client.request(req))?;
    let resp = rt.block_on(hyper::body::to_bytes(resp))?;

    // fetch the first few characters of the response bytes 
    Ok(resp)
}