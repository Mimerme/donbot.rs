mod don_bot;

//Currently working on:
//Getting the auto stitcher up and running
//  - idea is to just run a single binary and it'll download, combine, and upload the video
//  - for uploading the video we'll need a config_oauth binary for the user to login and get the
//  token
//  - take a relaxing day and go through all of the Clippy warnings

use ini::Ini;
use don_bot::twitch_core::{download_clip, get_helix_top_clips, Twitch_Clip};
use don_bot::gstreamer::{run_pipeline, stitch_videos_pipeline};
use don_bot::youtube_core::{upload_video};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Date, Utc, NaiveDate, DateTime, Datelike};

pub fn main() {
    println!("Loading from Ini file");

    // Load the coonfig values for the module
    let cfg = Ini::load_from_file("config.ini").unwrap();
    let auto_stitcher = cfg.section(Some("auto_stitch")).unwrap();
    let GAME_ID : &str = auto_stitcher.get("GAME_ID").unwrap();
    let DOWNLOAD_DIR : &str = auto_stitcher.get("DOWNLOAD_DIR").unwrap(); 

    //Some prints to make sure the values were read properly
    println!("Getting Twitch clips from '{}'", GAME_ID);
    println!("Downloading the clips to: {}", DOWNLOAD_DIR);


    // Get a deserialized JSON response fo the top clips on twitch
    let client = reqwest::blocking::Client::new();

    let current_date = Utc::now().date().naive_utc();
    //TODO: lol this is wrong
    let clips = get_helix_top_clips(&client, GAME_ID.to_string(), DateTime::from_utc(
            NaiveDate::from_ymd(current_date.year(), current_date.month(), current_date.day() - 1).and_hms(0,0,0), Utc) ,Utc::now()).unwrap();
    
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

    	download_clip(&client, &clip.mp4_url, &format!("{}{}/", DOWNLOAD_DIR, in_ms.to_string()), &filename);
        
        // build the list of local files to concat
        mp4s_to_concat.push(format!("{}{}/{}", DOWNLOAD_DIR, in_ms.to_string(), filename)); 
    }

    println!("Clips finished downloading. Constructing the stitching pipeline...");
    let concat_pipeline = stitch_videos_pipeline(mp4s_to_concat, "/home/mimerme/projects/donbot.rs/downloads/output.mp4".to_string(),60, 44100).unwrap();
    println!("Running the concatnation pipeline...");
    run_pipeline(concat_pipeline);
    //println!("Uploading the video...");
    //let res = upload_video(cfg, &"/home/mimerme/projects/donbot.rs/downloads/output.mp4".to_string()).unwrap(); 
    //println!("Response: {:?}", res);
}

fn filter_filename(filename_in : &mut String){
		const FILENAME_WHITELIST : &str = 
			"-_.() abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
		filename_in.retain(|x| {FILENAME_WHITELIST.contains(x)});
}
