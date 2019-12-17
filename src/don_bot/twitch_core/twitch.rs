//static mut client : Option<reqwest::Client> = None;
use reqwest;

/// Synchronous functions
pub fn download_clip(client : reqwest::Client) {
}

pub fn get_helix_top_clips(client : reqwest::blocking::Client, game_id : String) -> Result<bool, reqwest::Error> {
    let res = client.get("https://api.twitch.tv/helix/clips?game_id=")
                    .send()?;

    let status = res.status().is_success();
    let body = res.text()?;
    println!("Body: \n\n{}", body);

    return Ok(status);
}
