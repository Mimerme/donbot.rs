//static mut client : Option<reqwest::Client> = None;
use reqwest;
use serde::{Deserialize};
use std::fs::{File, write};
use chrono::{DateTime, Utc};
use crate::don_bot::utils::{filter_filename};
use futures::prelude::*;
use futures::executor::block_on;
use crate::don_bot::error::{DonBotResult, DBError};
use ini::Ini;
use std::io::{Read, copy};
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, DiskTokenStorage, FlowType, GetToken, Token};

const HELIX_ENDPOINT : &str = "https://api.twitch.tv/helix/{}";
const V5_ENDPOINT : &str= "https://api.twitch.tv/kraken/{}";
const CDN_ENDPOINT : &str = "https://clips-media-assets2.twitch.tv/";
//TODO: Support loading credentials from file
const CLIENT_ID : &str = "s0w8u8kr3e0s0mqgnzhyoom0bh7jzc";
const TOKEN_FILE : &str = ".oauth-token-twitch";
const SCOPES : [&str; 1] = ["clips:edit"];


#[derive(Deserialize)]
pub struct Twitch_Clip {
	pub id: String,
	pub url: String,
	pub embed_url: String,
	pub broadcaster_id: String,
	pub creator_id: String,
	pub creator_name: String,
	pub video_id: String,
	pub game_id: String,
	pub language:String,
	pub title: String,
	pub view_count: u32,
	pub created_at: String,
	pub thumbnail_url: String,
	#[serde(skip_deserializing)]
	pub mp4_url : String
}


pub struct TwitchClient {
    client : reqwest::blocking::Client
}

impl TwitchClient {
    pub fn new(cfg : &ini::Ini) -> TwitchClient{
        TwitchClient { client : reqwest::blocking::Client::new()}
    }

    fn init(){
        let secret = gen_application_secret(cfg);

        //Store our oauth token as '.oauth-token' in the working directory
        let oauthtoken = DiskTokenStorage::new(TOKEN_FILE.to_string()).unwrap();

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                  hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                                  oauthtoken, Some(FlowType::InstalledInteractive));
    }

    // has_delay specifies whether or not to account for the broadcaster viewer delay
    // it should be true when the viewer creates the clip and should be false when the streamer
    // creates the clip
    pub fn create_clip(&self, Stream, broadcaster_id : String, has_delay : bool) -> DonBotResult<()> {
        println!("Start time: {}", start_time.to_rfc3339());
        println!("End time: {}", end_time.to_rfc3339());
        let res = self.client.get("https://api.twitch.tv/helix/clips")
        				.query(&[("game_id", game_id),
                                 ("started_at", start_time.to_rfc3339()),
                                 ("ended_at", end_time.to_rfc3339())])
        				.header("Client-ID", CLIENT_ID)
                        .send()?;
    
    
        let status = res.status().is_success();
   
    
        //NOTE: .body() consumes the owernship of the response
        let body = res.text()?;
        if status != true {
        	return Err(Box::new(DBError::new(&body)));
        }
    
      	/* Structs for Helix specific JSON desrialization. */ 
      	/* Prefer fixed size stuff cuz Rust                */
    	#[derive(Deserialize)]
    	struct Helix_Pagination {
    		cursor: String
    	}
    
    	#[derive(Deserialize)]
    	struct Helix_Response {
    		data: Vec<Twitch_Clip>,
    		pagination: Helix_Pagination
    	}
    
    	//Iterate through all the returned Clips and download them
      	//TODO: Proper error handling
      	let mut json : Helix_Response = serde_json::from_str(&body).unwrap();
      	for mut clip in &mut (json.data) {
            let left_half = (clip.thumbnail_url.split("-preview-").collect::<Vec<&str>>())[0];
            let id = left_half.split("https://clips-media-assets2.twitch.tv/").collect::<Vec<&str>>()[1];
    
      		clip.mp4_url = format!("{}{}.mp4", CDN_ENDPOINT, id);
      	}
    
        return Ok(json.data);


    // Downloads multiple clips asyncronysoly (blah)
    /*pub fn download_clips(&self, urls : Vec<String>, download_dir : &str) -> DonBotResult<()> {
       //let futures : Vec<Future> = Vec::new();

       //create the 'downloads' directory if it doesn't exist
       std::fs::create_dir_all(download_dir);

       for url in urls {
    	    // Make the network request and setup the callbacks
            let future = self.client.get(&url).send().then(|res| async {

               //Format the path name and write the bytes
               let mut fname = url.to_string();
          	   filter_filename(&mut fname);
               let path = format!("{}{}{}", download_dir, "/", &fname);

               write(path, res.unwrap().bytes().await.unwrap());
            });
        }
        Ok(())
    }*/
 

    // Downloads a single clip and blocks the thread
    pub fn download_clip(&self, url : &str, download_dir : &str, filename : &str) -> DonBotResult<()> {
    	// Make the network request and unwrap the response
    	let mut res = self.client.get(url).send()?;
    
        //create the 'downloads' directory if it doesn't exist
        std::fs::create_dir_all(download_dir);

	    let mut dest : File = {
	    	//println!("file to download: '{}'", fname);
	    	let mut fname = filename.to_string();
	    	filter_filename(&mut fname);
            let path = format!("{}{}{}", download_dir, "/", &fname);
            //println!("will be located under: '{:?}'", path);
            File::create(&path).unwrap()
        };

        // Copy the contents from the response into the destination
        copy(&mut res, &mut dest);
    
        Ok(())
    }
    
    //TODO: Proper error handling
    pub fn get_helix_top_clips(&self, game_id : String, start_time : DateTime<Utc>, end_time : DateTime<Utc>) -> DonBotResult<Vec<Twitch_Clip>> {
        println!("Start time: {}", start_time.to_rfc3339());
        println!("End time: {}", end_time.to_rfc3339());
        let res = self.client.get("https://api.twitch.tv/helix/clips")
        				.query(&[("game_id", game_id),
                                 ("started_at", start_time.to_rfc3339()),
                                 ("ended_at", end_time.to_rfc3339())])
        				.header("Client-ID", CLIENT_ID)
                        .send()?;
    
    
        let status = res.status().is_success();
   
    
        //NOTE: .body() consumes the owernship of the response
        let body = res.text()?;
        if status != true {
        	return Err(Box::new(DBError::new(&body)));
        }
    
      	/* Structs for Helix specific JSON desrialization. */ 
      	/* Prefer fixed size stuff cuz Rust                */
    	#[derive(Deserialize)]
    	struct Helix_Pagination {
    		cursor: String
    	}
    
    	#[derive(Deserialize)]
    	struct Helix_Response {
    		data: Vec<Twitch_Clip>,
    		pagination: Helix_Pagination
    	}
    
    	//Iterate through all the returned Clips and download them
      	//TODO: Proper error handling
      	let mut json : Helix_Response = serde_json::from_str(&body).unwrap();
      	for mut clip in &mut (json.data) {
            let left_half = (clip.thumbnail_url.split("-preview-").collect::<Vec<&str>>())[0];
            let id = left_half.split("https://clips-media-assets2.twitch.tv/").collect::<Vec<&str>>()[1];
    
      		clip.mp4_url = format!("{}{}.mp4", CDN_ENDPOINT, id);
      	}
    
        return Ok(json.data);
    }
} 

pub fn config_oauth(cfg : &Ini) -> Result<Token, String>{
    println!("===DON'T FORGET TO REMOVE THE EXTRA COMMA SOME TERMINALS COPY AT THE END OF THE URL===");
    let secret = gen_application_secret(cfg);

    let oauthtoken = DiskTokenStorage::new(TOKEN_FILE.to_string()).unwrap();

    let mut auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                  hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                                  oauthtoken, Some(FlowType::InstalledInteractive));

    return auth.token(&SCOPES).map_err(|_| "Owoopsy woopsy in oawth towken 0w0".to_string());
}

fn gen_application_secret(cfg : &Ini) -> ApplicationSecret {
    let client_id = cfg.section(Some("twitch")).unwrap().get("CLIENT_ID").unwrap();
    let client_secret = cfg.section(Some("twitch")).unwrap().get("CLIENT_SECRET").unwrap();

    //println!("ID: {}", client_id.to_string());
    //println!("SECRET: {}", client_secret.to_string());

    let secret: ApplicationSecret = 
        ApplicationSecret {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            token_uri: "https://id.twitch.tv/token".to_string(),
            auth_uri: "https://id.twitch.tv/oauth2/authorize".to_string(),
            redirect_uris: vec!["urn:ietf:wg:oauth:2.0:oob".to_string(), "http://localhost".to_string()],
            project_id: Some("donbot-250400".to_string()),
            client_email: None,
            auth_provider_x509_cert_url: None,
            client_x509_cert_url: None
        };

    return secret;
}


