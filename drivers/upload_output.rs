use std::fs;
use std::path::Path;
use ini::Ini;
use don_bot::gstreamer::{stitch_videos};
use don_bot::youtube_core::{upload_video};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Date, Utc, NaiveDate, DateTime, Datelike};


pub fn main(){
    let cfg = Ini::load_from_file("config.ini").unwrap();
    let auto_stitcher = cfg.section(Some("auto_stitch")).unwrap();
    let GAME_ID : &str = auto_stitcher.get("GAME_ID").unwrap();
    let DOWNLOAD_DIR : &str = auto_stitcher.get("DOWNLOAD_DIR").unwrap(); 
    let OUTPUT_FILE : &str = cfg.section(Some("gstreamer")).unwrap().get("OUTPUT_FILE").unwrap();

   let res = upload_video(&cfg, &OUTPUT_FILE.to_string(), 
                           Some("Daily Dota 2 Twitch Highlights".to_string()), 
                           Some("This is an automatically generated video by DonBot.".to_string())).unwrap(); 
    println!("Response: {:?}", res);
}

