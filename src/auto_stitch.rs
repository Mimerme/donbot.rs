mod don_bot;

use don_bot::twitch_core::{download_clip, get_helix_top_clips, Twitch_Clip};
use std::time::{SystemTime, UNIX_EPOCH};

/// Set the game_id before building
static GAME_ID : &str = "29595";
static DONWLOAD_DIR : &str = "/home/mimerme/projects/donbot.rs/downloads/";

pub fn main() {
    println!("Getting Twitch clips from '{}'", GAME_ID);

    let client = reqwest::blocking::Client::new();
    let clips = get_helix_top_clips(&client, GAME_ID.to_string()).unwrap();

    //Get a time stamp to create the folder name with
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards?!?!");
    let in_ms = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    for clip in clips.iter() {
    	let mut filename = format!("{}.mp4", clip.title);
        filter_filename(&mut filename);
    	download_clip(&client, &clip.mp4_url, &format!("{}{}/", DONWLOAD_DIR, in_ms.to_string()), &filename);
    }
}

fn filter_filename(filename_in : &mut String){
		const FILENAME_WHITELIST : &str = 
			"-_.() abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
		filename_in.retain(|x| {FILENAME_WHITELIST.contains(x)});
}
