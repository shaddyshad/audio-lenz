extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_speech1 as speech1;

use speech1::{Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use speech1::Speech;

fn main() {
    // Get an application authenticator instance containing client_id and client_secret 
    let secret : ApplicationSecret = Default::default();

    // create a connector 
    let connector = hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()));

    // instantiate the authenticator 
    let auth = Authenticator::new(
        &secret, DefaultAuthenticatorDelegate, 
        connector, 
        <MemoryStorage as Default>::default(), None);

    let hub_connector = hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()));

    // create a hub 
    let hub = Speech::new(hub_connector, auth);

    // get a list of operations 
    let result = hub.operations().list()
                .page_token("et")
                .page_size(-18)
                .name("kasd")
                .filter("accusam")
                .doit();

    
    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled 
            | Error::UploadSizeLimitExceeded(_,_)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_,_) => println!("{}", e),
        },
        Ok(res) => println!("Success {:?}", res)
    }
}
