use ini::Ini;

#[test]
//This test depends on the Zain twitch clips, so run twitch_core::test_download_clip first
fn test_upload_video(){
    use super::*;

    //NOTE: This is here so I don't accidently useup my API quota by testing too much
    return;

    let cfg = Ini::load_from_file("config.ini").unwrap();
    //println!("Response: {:?}", upload_video(cfg, "/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4").unwrap()); 
}
