/// This module is used for using pre-written gstreamer pipelines
pub mod gstreamer;


/// FUNCTIONS
/// Download a twitch clip
///
/// * 'path' - file path to save the video as. Note: must be mp4
/// * 'mp4url' - url to the mp4 video file. Found in the source of the webpage
//pub use gstreamer::stitch_videos;

#[cfg(test)]
mod tests;
