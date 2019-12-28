extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_youtube3 as youtube3;

use std::fs;
use ini::Ini;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, DiskTokenStorage, FlowType};
use youtube3::{Result, Error};
use youtube3::{YouTube, Video};
use hyper::client::response::Response;

pub fn upload_video(cfg : Ini, video_path : String) -> Result<(Response, Video)> {
    println!("Starting video upload");

    let client_id = cfg.section(Some("youtube")).unwrap().get("CLIENT_ID").unwrap();
    let client_secret = cfg.section(Some("youtube")).unwrap().get("CLIENT_SECRET").unwrap();

    println!("ID: {}", client_id.to_string());
    println!("SECRET: {}", client_secret.to_string());

    let secret: ApplicationSecret = 
        ApplicationSecret {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            redirect_uris: vec!["urn:ietf:wg:oauth:2.0:oob".to_string(), "http://localhost".to_string()],
            project_id: Some("donbot-250400".to_string()),
            client_email: None,
            auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
            client_x509_cert_url: None
        };


    let oauthtoken = DiskTokenStorage::new(&".oauth-token".to_string()).unwrap();

    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate, 
                                  hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                                  oauthtoken, Some(FlowType::InstalledRedirect(80)));


    let mut hub = YouTube::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);

    println!("Hub created");

    return Ok(hub.videos().insert(Video::default())
                .part("id")
                .upload(fs::File::open("/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4").unwrap(), "video/mp4".parse().unwrap())?);

}
