mod don_bot;

use std::fs;
use std::path::Path;
use ini::Ini;
use don_bot::twitch_core::{download_clip, get_helix_top_clips, Twitch_Clip};
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

    let res = upload_video(&cfg, &OUTPUT_FILE.to_string(), "", None).unwrap(); 
    println!("Response: {:?}", res);
}

