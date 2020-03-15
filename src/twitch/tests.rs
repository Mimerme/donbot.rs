use chrono::{Date, Utc, NaiveDate, DateTime};
use ini::Ini;

#[test]
fn test_get_user_id(){
    use super::*;

    let cfg = Ini::load_from_file("config.ini").unwrap();
    let twitch_client = TwitchClient::new(&cfg);
    let res = twitch_client.get_user_id("mimerme".to_string());
   
    let e = "57869370".to_string();
    match res {
        Result::Ok(e) => {assert!(true)},
        Result::Ok(_) => {assert!(false)},
        Result::Err(_) =>  assert!(false)
    }
}

#[test]
fn test_get_helix_clips(){
    use super::*;

    let cfg = Ini::load_from_file("config.ini").unwrap();
    let twitch_client = TwitchClient::new(&cfg);
    let res = twitch_client.get_helix_top_clips("29595".to_string(), DateTime::from_utc(NaiveDate::from_ymd(2019, 1, 1).and_hms(0,0,0), Utc) ,Utc::now());
    
    match res {
        Result::Ok(res) => assert!(true),
        Result::Err(_) =>  assert!(false)
    }
}

#[test]
fn test_create_clip(){
    use super::*;

    let cfg = Ini::load_from_file("config.ini").unwrap();
    let twitch_client = TwitchClient::new(&cfg);
    //Leffen's channel id
    let res = twitch_client.create_clip("53831525".to_string(), false);    
    match res {
        Result::Ok(res) => assert!(true),
        Result::Err(_) =>  assert!(false)
    }
}


#[test]
fn test_download_clip(){
    use super::*;

    let cfg = Ini::load_from_file("config.ini").unwrap();
    let twitch_client = TwitchClient::new(&cfg);
 
    // Clip of Zain explaining the Marth Falco matchup lul
    let res = twitch_client.download_clip("https://clips-media-assets2.twitch.tv/AT-cm|585662124.mp4", 
        "./downloads/", "test.mp4");
    
    match res {
        Result::Ok(_) => assert!(true),
        Result::Err(_) =>  assert!(false)
    }

}

