extern crate audio_lenz;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde;

use hyper::{Client, Request, Method, Body, body};
use hyper_rustls::HttpsConnector;
use tokio::runtime::Runtime;

use std::io::prelude::*;

use std::fs::File;
use serde::{Serialize, Deserialize};

const HOST: &'static str = "https://eastus.stt.speech.microsoft.com/speech/recognition/conversation/cognitiveservices/v1?language=en-US";
const KEY: &'static str = "4a1c4999aff742b6889c5b1a5bfdbfe0";


#[derive(Debug, Serialize, Deserialize)]
struct Recognition {
    #[serde(alias="RecognitionStatus")]
    status: String,
    #[serde(alias="DisplayText")]
    text: String,
    #[serde(alias="Duration")]
    duration: usize,
    #[serde(alias="Offset")]
    offset: usize
}


fn main(){
    let https = HttpsConnector::new();
    let client: Client<_, Body> = Client::builder().build(https);
    

    // open the audio file 
    let mut audio_file = File::open("/home/ank3r/Documents/ml/audio-lenz/samples/sample-1.ogg").unwrap();
    let mut buf = Vec::new();

    audio_file.read_to_end(&mut buf).unwrap();

    let req = Request::builder()
            .uri(HOST)
            .header("Ocp-Apim-Subscription-Key", KEY)
            .method(Method::POST)
            .header("Content-Type", "audio/ogg; codecs=opus")
            .body(Body::from(buf))
            .unwrap();

    // create a new runtime 
    let mut rt = Runtime::new().unwrap();

    // make a post request 
    let res = rt.block_on(client.request(req)).unwrap();

    // read to a buffer 
    let res = rt.block_on(body::to_bytes(res)).unwrap();

    let parsed: Recognition = serde_json::from_slice(&res).unwrap();
    println!("{:#?}", parsed);
    
}