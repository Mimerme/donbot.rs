use chrono::{Date, Utc, NaiveDate, DateTime};

#[test]
fn test_get_helix_clips(){
    use super::*;

    let res = get_helix_top_clips(&reqwest::blocking::Client::new(), "29595".to_string(), DateTime::from_utc(NaiveDate::from_ymd(2019, 1, 1).and_hms(0,0,0), Utc) ,Utc::now());
    
    match res {
        Result::Ok(res) => assert!(true),
        Result::Err(_) =>  assert!(false)
    }
}

#[test]
fn test_download_clip(){
    use super::*;

    // Clip of Zain explaining the Marth Falco matchup lul
    let res = download_clip(&reqwest::blocking::Client::new(), 
        "https://clips-media-assets2.twitch.tv/AT-cm|585662124.mp4", 
        "./downloads/", "test.mp4");
    
    match res {
        Result::Ok(_) => assert!(true),
        Result::Err(_) =>  assert!(false)
    }

}
