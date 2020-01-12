mod don_bot;

use ini::Ini;
use don_bot::youtube_core::config_oauth;
pub fn main() {
    let cfg = Ini::load_from_file("config.ini").unwrap();
    let res = config_oauth(&cfg);

    println!("Response: {:?}", res.unwrap());
}
