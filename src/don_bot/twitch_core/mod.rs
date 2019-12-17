pub mod twitch;
/// This module is used for interfacing with Twitch

/// Download a twitch clip
///
/// * 'path' - file path to save the video as. Note: must be mp4
/// * 'mp4url' - url to the mp4 video file. Found in the source of the webpage
pub use twitch::download_clip;

pub use twitch::get_helix_top_clips;

#[cfg(test)]
mod tests;
