use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};
use don_bot::twitch::TwitchClient;
use std::env;
use ini::Ini;

fn main() {
    let args : Vec<String> = env::args().collect();
    let login = args[1].clone();

    let cfg = Ini::load_from_file("config.ini").unwrap();
    let twitch_client = TwitchClient::new(&cfg);
    let id = twitch_client.get_user_id(login.to_string()).unwrap();


    println!("{} has a user id of {}", login, id);

    //NOTE: this stops the LControl key from being used. (On X11)
    //BUG?
    //TODO: implement the copy attribute to twitch_client
    LControlKey.bind(move || {
        while LControlKey.is_pressed() {
            if XKey.is_pressed(){
                let cfg = Ini::load_from_file("config.ini").unwrap();
                let twitch_client = TwitchClient::new(&cfg);

                let clip_id = twitch_client.create_clip(id.to_string(), false).unwrap();
                println!("Created a clip with id: {}", clip_id);
                break;
            }
        }
    });

    // Call this to start listening for bound inputs.
    handle_input_events();
}
