mod don_bot;
//Driver for generating a gstreamer encoding profile based on an input video
//NOTE: An encoding profile specifies parameters such as the compression, fps, resolution, and
//sample rate for the output video file.
use std::env;
use don_bot::gstreamer::{generate_encoding_profile};

fn main(){
    let args : Vec<String> = env::args().collect();
    
    let source_uri = &args[1];
    let output_file = &args[2];

    println!("Source File: {}", source_uri);
    println!("Encoding Output: {}", output_file);

    generate_encoding_profile(source_uri, output_file);
}
