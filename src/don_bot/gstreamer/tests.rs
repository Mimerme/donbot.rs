use ini::Ini;

#[test]
fn test_generate_overlay(){}

#[test]
fn test_concat_clips(){
    use super::*;

    //let cfg = Ini::load_from_file("config.ini").unwrap();
    //let pipeline = stitch_videos_pipeline(vec!["/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4".to_string(),
    //            "/home/mimerme/Downloads/SampleVideo_1280x720_1mb.mp4".to_string()], &cfg).unwrap();
   
    //NOTE: many errors won't arise in the pipeline until we run it
    //run_pipeline(pipeline);

}


#[test]
fn test_benchmark(){}


// Downloads. Depends on twitch_core for downloading videos
fn prepare_test_env(){
    use crate::don_bot::twitch_core::{download_clip};

    let const test_videos = vec!["http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                                 "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
                                 "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4"];

}
