use ini::Ini;
use don_bot::youtube_core;
use don_bot::twitch_core::TwitchClient;
use std::env;

pub fn main() {
    let args : Vec<String> = env::args().collect();

    let cfg = Ini::load_from_file("config.ini").unwrap();
    match args[1].as_str() {
       "youtube" => {
            println!("Generating YouTube OAuth Token");
            let res = youtube_core::config_oauth(&cfg);
            println!("Response: {:?}", res.unwrap());
       },
       "twitch" => {
            println!("Generating Twitch OAuth Token");
            let mut twitchclient = TwitchClient::new(&cfg);
            //let res = twitchclient.config_oauth(); 
            //println!("Response: {:?}", res.unwrap());
       },
       _ => {panic!("No.");}
    }
}
