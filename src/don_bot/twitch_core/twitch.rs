//static mut client : Option<reqwest::Client> = None;
use reqwest;
use serde::{Deserialize};
use std::fs::{File, write};
use chrono::{DateTime, Utc};
use crate::don_bot::utils::{filter_filename};
use futures::prelude::*;
use futures::executor::block_on;
use crate::don_bot::error::{DonBotResult, DBError};

const HELIX_ENDPOINT : &str = "https://api.twitch.tv/helix/{}";
const V5_ENDPOINT : &str= "https://api.twitch.tv/kraken/{}";
const CDN_ENDPOINT : &str = "https://clips-media-assets2.twitch.tv/";
//TODO: Support loading credentials from file
const CLIENT_ID : &str = "s0w8u8kr3e0s0mqgnzhyoom0bh7jzc";


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
    client : reqwest::Client
}

impl TwitchClient {
    pub fn new() -> TwitchClient{
        TwitchClient { client : reqwest::Client::new()}
    }

    /*
    // Downloads multiple clips asyncronysoly (blah)
    pub fn download_clips(&self, urls : Vec<String>, download_dir : &str) -> DonBotResult<()> {
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
    }
 

    // Downloads a single clip and blocks the thread
    pub fn download_clip(&self, url : &str, download_dir : &str, filename : &str) -> DonBotResult<()> {
    	// Make the network request and unwrap the response
    	let mut res = block_on(self.client.get(url).send())?;
    
        //create the 'downloads' directory if it doesn't exist
        std::fs::create_dir_all(download_dir);

        let mut fname = filename.to_string();
    	filter_filename(&mut fname);
        let path = format!("{}{}{}", download_dir, "/", &fname);

        let bytes = block_on(res.bytes()).ok_or(DBError::new("problem reading bytes"));;
        
        // Copy the contents from the response into the destination
        write(path, bytes);
    
        Ok(())
    }
    
    //TODO: Proper error handling
    pub fn get_helix_top_clips(client : &reqwest::blocking::Client, game_id : String, start_time : DateTime<Utc>, end_time : DateTime<Utc>) -> DonBotResult<Vec<Twitch_Clip>> {
        println!("Start time: {}", start_time.to_rfc3339());
        println!("End time: {}", end_time.to_rfc3339());
        let res = block_on(client.get("https://api.twitch.tv/helix/clips")
        				.query(&[("game_id", game_id),
                                 ("started_at", start_time.to_rfc3339()),
                                 ("ended_at", end_time.to_rfc3339())])
        				.header("Client-ID", CLIENT_ID)
                        .send())?;
    
    
        let status = res.status().is_success();
        if status != true {
        	return Err("Oof".to_string());
        }
    
    
        //NOTE: .body() consumes the owernship of the response
        let body = res.text()?;
    
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
    }*/
}

