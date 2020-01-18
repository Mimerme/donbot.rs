/// This module is used for interfacing with Twitch
pub mod twitch;

/// STRUCTS
pub use twitch::Twitch_Clip;


/// FUNCTIONS
/// Download a twitch clip
///
/// * 'path' - file path to save the video as. Note: must be mp4
/// * 'mp4url' - url to the mp4 video file. Found in the source of the webpage
//pub use twitch::download_clip;

/// Returns Vec<Helix_Clip>
//pub use twitch::get_helix_top_clips;

pub use twitch::TwitchClient;

#[cfg(test)]
mod tests;
