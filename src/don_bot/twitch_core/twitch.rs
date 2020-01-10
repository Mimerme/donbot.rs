//static mut client : Option<reqwest::Client> = None;
use reqwest;
use serde::{Deserialize};
use std::fs::File;
use std::io::copy;
use chrono::{DateTime, Utc};

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

// Blocking
pub fn download_clip(client : &reqwest::blocking::Client, url : &str, download_dir : &str, filename : &str) -> Result<(), reqwest::Error> {
	// Make the network request and unwrap the response
	let mut res = client.get(url).send()?;

    //create the 'downloads' directory if it doesn't exist
    std::fs::create_dir_all(download_dir);

	// Create a new File struct
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
pub fn get_helix_top_clips(client : &reqwest::blocking::Client, game_id : String, start_time : DateTime<Utc>, end_time : DateTime<Utc>) -> Result<Vec<Twitch_Clip>, String> {
    println!("Start time: {}", start_time.to_rfc3339());
    println!("End time: {}", end_time.to_rfc3339());
    let res = client.get("https://api.twitch.tv/helix/clips")
    				.query(&[("game_id", game_id),
                             ("started_at", start_time.to_rfc3339()),
                             ("ended_at", end_time.to_rfc3339())])
    				.header("Client-ID", CLIENT_ID)
                    .send().unwrap();


    let status = res.status().is_success();
    if status != true {
    	return Err("Oof".to_string());
    }


    //NOTE: .body() consumes the owernship of the response
    let body = res.text().unwrap();
  	//println!("Body: \n\n{}", body);

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

/// This is a function that filters out invalid filename characters
/// Filters using a Whitelist approach. Weird OSes have weird stuff going on
/// Approach taken from: https://stackoverflow.com/a/295146
/// Valid characters: -_.() abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789
fn filter_filename(filename_in : &mut String){
		const FILENAME_WHITELIST : &str = 
			"-_.() abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
		filename_in.retain(|x| {FILENAME_WHITELIST.contains(x)});
}
