mod don_bot;

use ini::Ini;
use don_bot::youtube_core::upload_video;
pub fn main() {
    let cfg = Ini::load_from_file("config.ini").unwrap();
    let res = upload_video(cfg, "test".to_string());

    match res {
        Ok(res) => println!("Response: {:?})", res),
        Err(res) => println!("Response: {:?})", res)
    };
}
