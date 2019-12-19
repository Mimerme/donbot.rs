//static mut client : Option<reqwest::Client> = None;
use reqwest;
use serde::{Deserialize};
use std::fs::File;
use std::io::copy;

const HELIX_ENDPOINT : &str = "https://api.twitch.tv/helix/{}";
const V5_ENDPOINT : &str= "https://api.twitch.tv/kraken/{}";
const CDN_ENDPOINT : &str = "https://clips-media-assets2.twitch.tv/";
//TODO: Support loading credentials from file
const CLIENT_ID : &str = "s0w8u8kr3e0s0mqgnzhyoom0bh7jzc";



#[derive(Deserialize)]
pub struct Twitch_Clip {
	id: String,
	url: String,
	embed_url: String,
	broadcaster_id: String,
	creator_id: String,
	creator_name: String,
	video_id: String,
	game_id: String,
	language:String,
	title: String,
	view_count: u32,
	created_at: String,
	thumbnail_url: String,
	#[serde(skip_deserializing)]
	mp4_url : String
}

// Blocking
pub fn download_clip(client : &reqwest::blocking::Client, url : &str, download_dir : &str, filename : &str) -> Result<(), reqwest::Error> {
	// Make the network request and unwrap the response
	let mut res = client.get(url).send()?;

	// Create a new File struct
	let mut dest : File = {
		// Not sure what these all do but o' well
		let fname = res
				.url()
				.path_segments()
		        .and_then(|segments| segments.last())
		        .and_then(|name| if name.is_empty() { None } else { Some(name) })
		        .unwrap_or("tmp.mp4");	


		println!("file to download: '{}'", fname);
		let mut filename = fname.to_string();
		filter_filename(&mut filename);
        let fname = format!("{}{}{}", download_dir, "/", &filename);
        println!("will be located under: '{:?}'", fname);
        File::create(&fname).unwrap()
    };

    // Copy the contents from the response into the destination
    // NOTE: Weird design cuz of 0-cost abstraction?
    copy(&mut res, &mut dest);

    Ok(())
}

//TODO: Proper error handling
pub fn get_helix_top_clips(client : &reqwest::blocking::Client, game_id : String) -> Result<Vec<Twitch_Clip>, String> {
    let res = client.get("https://api.twitch.tv/helix/clips")
    				.query(&[("game_id", game_id)])
    				.header("Client-ID", CLIENT_ID)
                    .send().unwrap();


    let status = res.status().is_success();
    if status != true {
    	return Err("Oof".to_string());
    }


    //NOTE: .body() consumes the owernship of the response
    let body = res.text().unwrap();
  	println!("Body: \n\n{}", body);

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
  		clip.mp4_url = format!("{}/{}.mp4", CDN_ENDPOINT, (clip.thumbnail_url.split("-preview-").collect::<Vec<&str>>())[0]);
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