mod don_bot;

use don_bot::twitch_chat::{TwitchChat, ChatMsg};
use notify_rust::Notification;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;

pub fn main(){
    let mut chat = TwitchChat::new("mimerme");

    chat.add_message_listener(Box::new(|msg : &ChatMsg| {
        Notification::new()
            .summary("Twitch Chat")
            .body("Waow you got a message!!!")
            .show();
        let device = rodio::default_output_device().unwrap();

        let file = File::open("sound.mp3").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
        return ();
    }));

    chat.connect();
}
