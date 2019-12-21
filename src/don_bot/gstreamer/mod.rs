
/// This module is used for using pre-determined gstreamer pipelines

// Notes for Mimerme
// gst-launch-1.0 filesrc location=/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin ! autovideosink;
// Pipeline Command: gst-launch-1.0 filesrc location={file_path} ! decodebin
// gst-launch-1.0 concat name=cat ! filesrc
// location=/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! cat.filesrc
// location=/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! cat.
// Why the fuck is this framework made 10x more complicated than it has be be : 12/20/2019

/// Example:
pub mod gstreamer;


/// FUNCTIONS

/// Stitch .mp4 files together
///
/// * 'mp4_pathes' - a vector of mp4 file paths on the binary's local machine
/// * 'output_path' - output file of the final video
/// * 'overlay_path' - path to
pub use gstreamer::stitch_videos;

/// Apply an overlay to a video file and outputs the result
/// * 'overlay_path' - a path to a _____ file
/// * 'src_path' - path to source mp4 file
/// * 'output_path' - 
// pub use gstreamer::apply_overlay;

#[cfg(test)]
mod tests;
