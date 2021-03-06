
/// This module is used for using pre-determined gstreamer pipelines

// Notes for Mimerme
// gst-launch-1.0 filesrc location=/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin ! autovideosink;
// Pipeline Command: gst-launch-1.0 filesrc location={file_path} ! decodebin
// gst-launch-1.0 concat name=cat ! filesrc
// location=/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! cat.filesrc
// location=/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! cat.
// Why the fuck is this framework made 10x more complicated than it has be be : 12/20/2019
// TODO: make gstreamer quickstart guide
//      - gstreamer is actually so good but youngin devs haven't really heard of it
//      - super simple concept. super easy to visulize and understand. documentation sux
//      - 12/24/2019

// Command for concatnating videos (though it only does the video stream, not the audio)
// gst-launch-1.0 concat name=c ! autovideosink  filesrc location=~/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin ! video/x-raw ! c. filesrc location=~/Downloads/SampleVideo_1280x720_1mb.mp4 ! decodebin ! video/x-raw ! c.

//
//gst-launch-1.0 concat name=c ! autovideosink  concat name=d ! audioconvert ! autoaudiosink  filesrc location=~/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin name=f1  f1. ! video/x-raw ! c. filesrc location=~/Downloads/SampleVideo_1280x720_1mb.mp4 ! decodebin name=f2  f2. ! video/x-raw ! c.  f1. ! audio/x-raw ! d.  f2.src_1 ! audio/x-raw ! d.

//gst-launch-1.0 mp4mux name=mux ! filesink location=ccc.mp4  concat name=c ! videoconvert  ! x264enc ! mux.video_0  concat name=d ! audioconvert ! faac ! aacparse ! mux.audio_0  filesrc location=~/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin name=f1  f1. ! video/x-raw ! c. filesrc location=~/Downloads/SampleVideo_1280x720_1mb.mp4 ! decodebin name=f2  f2. ! video/x-raw ! c.  f1. ! audio/x-raw ! d.  f2. ! audio/x-raw ! d.

//gst-launch-1.0 mpegtsmux name=mux ! filesink location=done.mp4  concat name=c ! videoconvert ! videoscale ! x264enc ! mux.  concat name=d ! audioconvert ! faac ! aacparse ! mux.  filesrc location=~/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin name=f1  filesrc location=~/Downloads/SampleVideo_1280x720_1mb.mp4 ! decodebin name=f2  f2. ! video/x-raw ! c.  f1. ! video/x-raw ! c.  f2. ! audioresample ! audio/x-raw,rate=44100 ! audioconvert ! audio/x-raw,channels=2 ! d.  f1. ! audio/x-raw ! d.

//last pipline prob
//gst-launch-1.0 mpegtsmux name=mux ! filesink location=lul.mp4  concat name=c ! videoconvert ! videoscale ! x264enc ! mux.  concat name=d ! audioconvert ! audiorate ! faac ! aacparse ! mux.  filesrc location=~/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin name=f1  filesrc location=~/Downloads/SampleVideo_1280x720_1mb.mp4 ! decodebin name=f2  f2. ! video/x-raw ! queue ! c.  f1. ! video/x-raw ! queue ! c.  f2. ! audioresample ! audio/x-raw,rate=44100 ! audioconvert ! audio/x-raw,channels=2 ! queue ! d.  f1. ! audio/x-raw ! queue ! d.


/// Example:
pub mod gst;

//TODO: Generalize this module to be more robust
// rn it the design is very rigid and fits only my use case

/// FUNCTIONS

/// Creates a new gstreamer pipeline and returns it
/// * 'name' - an option for the name of the pipline
//pub use gst::create_pipeline;

/// Runs a specified pipeline
/// * 'pipeline' - The pipeline to run. NOTE: Its ownership is consumed

/// Creates a bunch of filesrc elements and returns them
/// * '&pipeline' - Reference of pipeline to add elements to
/// * 'paths' - Vec<String> of files and their paths
//pub use gst::create_filesrcs;

/// Insert a concat element into the pipeline
/// * '&pipeline' - Reference of pipeline to add elements to
/// * 'filesrcs' - Vec<gstreamer::Element> of filesrc elements to concat
pub use gst::stitch_videos;
pub use gst::generate_encoding_profile;

/// Outputs a video by attatching a filesink element
/// * '&pipeline' - Reference of pipeline to add elements to
/// * 'output_path' - Path to the output file
/// * 'prev_elem' - element to attach the filesink to
//pub use gst::output_video;

/// Apply an overlay to a video file and outputs the result
/// * 'overlay_path' - a path to a _____ file
/// * 'src_path' - path to source mp4 file
/// * 'output_path' - 
// pub use gstreamer::apply_overlay;

//pub use gst::

#[cfg(test)]
mod tests;
