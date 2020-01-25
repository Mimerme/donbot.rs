//Currently working on:
//Getting the auto stitcher up and running
//  - idea is to just run a single binary and it'll download, combine, and upload the video
//  - for uploading the video we'll need a config_oauth binary for the user to login and get the
//  token
//  - take a relaxing day and go through all of the Clippy warnings


use std::path::Path;
use std::{fs, result};
use don_bot::twitch::TwitchClient;
use don_bot::gstreamer::{stitch_videos};
use don_bot::youtube::{upload_video};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Date, Utc, NaiveDate, DateTime, Datelike, Duration};
use ini::Ini;

pub fn main() {
    println!("Loading from Ini file");

    // Load the coonfig values for the module
    let cfg = Ini::load_from_file("config.ini").unwrap();
    let auto_stitcher = cfg.section(Some("auto_stitch")).unwrap();
    let GAME_ID : &str = auto_stitcher.get("GAME_ID").unwrap();
    let DOWNLOAD_DIR : &str = auto_stitcher.get("DOWNLOAD_DIR").unwrap(); 
    let OUTPUT_FILE : &str = cfg.section(Some("gstreamer")).unwrap().get("OUTPUT_FILE").unwrap();

    //Some prints to make sure the values were read properly
    println!("Getting Twitch clips from '{}'", GAME_ID);
    println!("Downloading the clips to: {}", DOWNLOAD_DIR);

    // Get a deserialized JSON response fo the top clips on twitch
    let client = reqwest::blocking::Client::new();

    let current_date = Utc::now();
    //TODO: lol this is wrong
    //let clips = get_helix_top_clips(&client, GAME_ID.to_string(), DateTime::from_utc(
    //        NaiveDate::from_ymd(current_date.year(), current_date.month(), current_date.day() - 1).and_hms(0,0,0), Utc) ,Utc::now()).unwrap();
    let twitch_client = TwitchClient::new(&cfg);

    let clips = twitch_client.get_helix_top_clips(GAME_ID.to_string(), current_date - Duration::days(1), current_date).unwrap();
    //Generate a time stamp to create the folder name with
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards?!?!");
    let in_ms = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    // Build the input arguments for gstreamer::stitch_videos_pipeline
    let mut mp4s_to_concat = Vec::<String>::new();

    for clip in clips.iter() {
        //Genearte a valid filename
    	let mut filename = format!("{}.mp4", clip.id);
        filter_filename(&mut filename);
      
        //Some print statements to make sure the program is running
        println!("Downloading {} to {}/{}", clip.title, filename, in_ms.to_string());
        println!("Source: {}", clip.mp4_url);

    	twitch_client.download_clip(&clip.mp4_url, &format!("{}{}/", DOWNLOAD_DIR, in_ms.to_string()), &filename);
        
        // build the list of local files to concat
        mp4s_to_concat.push(format!("{}{}/{}", DOWNLOAD_DIR, in_ms.to_string(), filename)); 
    }

    println!("Clips finished downloading. Running the concatnation pipeline...");
    let concat_pipeline = stitch_videos(mp4s_to_concat, &cfg).unwrap();

    println!("Uploading the video...");
    let res = upload_video(&cfg, &OUTPUT_FILE.to_string(), 
                           Some(format!("Daily Dota 2 Twitch Highlights : {}", current_date.format("%Y-%m-%d").to_string()).to_string()), 
                           Some("This is an automatically generated video by DonBot.".to_string())).unwrap(); 
    println!("Response: {:?}", res);
}

fn filter_filename(filename_in : &mut String){
		const FILENAME_WHITELIST : &str = 
			"-_.() abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
		filename_in.retain(|x| {FILENAME_WHITELIST.contains(x)});
}

#[test]
pub fn test_stitching(){
    let cfg = Ini::load_from_file("config.ini").unwrap();
    let auto_stitcher = cfg.section(Some("auto_stitch")).unwrap();
    let GAME_ID : &str = auto_stitcher.get("GAME_ID").unwrap();
    let DOWNLOAD_DIR : &str = auto_stitcher.get("DOWNLOAD_DIR").unwrap(); 

    let mp4s_to_concat = files_within_dir(Path::new("/home/mimerme/projects/donbot.rs/downloads/1578101953659"));
    println!("{:?}", mp4s_to_concat);

    let concat_pipeline = stitch_videos(mp4s_to_concat, &cfg).unwrap();
}

fn files_within_dir(dir : &Path) -> Vec<String> {
    let mut files : Vec<String> = Vec::<String>::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    return files;

}
