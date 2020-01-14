extern crate gstreamer as gst;
use gstreamer::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

extern crate gstreamer_pbutils as gst_pbutils;
use gst_pbutils::prelude::*;

use ini::Ini;
use std::path::Path;

use crate::don_bot::error::{DonBotResult, DBError};
//use crate::don_bot::utils::don_bot_err;

// Returns a path to the output file
pub fn stitch_videos(clips : Vec<String>, cfg : &Ini) -> DonBotResult<&Path>{
    ges::init()?;

    // Load the configuration files
    let gstreamer_section = cfg.section(Some("gstreamer")).ok_or(DBError::new("Missing gstreamer section"))?;
    let output_file = gstreamer_section.get("OUTPUT_FILE").ok_or(DBError::new("Missing output file"))?;
    let encoding_profile_file = gstreamer_section.get("ENCODING_TARGET").ok_or(DBError::new("Missing encoding profile"))?;
    // Begin by creating a timeline with audio and video tracks
    let timeline = ges::Timeline::new_audio_video();
    // Create a new layer that will contain our timed clips.
    let layer = timeline.append_layer();
    //Create a pipeline with the timeline
    let pipeline = ges::Pipeline::new();
    pipeline.set_timeline(&timeline)?;

    let mut current_time = gst::ClockTime::from_seconds(0);
    for clip in clips {
       let clip = ges::UriClip::new(&format!("file://{}", clip))?;

        // Retrieve the asset that was automatically used behind the scenes, to
       // extract the clip from.
       let asset = clip.get_asset().unwrap();
       let duration = asset
           .downcast::<ges::UriClipAsset>()
           .unwrap()
           .get_duration();
       
       // The inpoint specifies where in the clip we start, the duration specifies
       // how much we play from that point onwards. Setting the inpoint to something else
       // than 0, or the duration something smaller than the clip's actual duration will
       // cut the clip.
       clip.set_inpoint(gst::ClockTime::from_seconds(0));
       clip.set_start(current_time);
       clip.set_duration(duration);
       current_time = current_time + duration; 
       layer.add_clip(&clip)?;
    }

    //Load the settings for encoding the final render
    let profiles = gst_pbutils::EncodingTarget::load_from_file(&encoding_profile_file)?.get_profiles();

    println!("Reading Encoding Target From: {}", &encoding_profile_file);
    pipeline.set_render_settings(&format!("file://{}", output_file), &profiles[0]);

    //Begin the render proccess
    pipeline.set_mode(ges::PipelineFlags::RENDER);
    run_pipeline(pipeline)?;

    return Ok(&Path::new(output_file));
}

fn run_pipeline(pipeline : ges::Pipeline) -> DonBotResult<()> {
    // Below is code to look over the pipeline bus.
    // Looks leik its mainly for debugging purposes but idk I just ripped it from here:
    // https://github.com/sdroege/gstreamer-rs/blob/be3c378f289683e8c0e7b7cfaff5dc74972bb074/examples/src/bin/playbin.rs    

    //pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "NOT_PLAYING");
    
    //TODO: Proper error handling
    pipeline.set_state(gstreamer::State::Playing)?;

    println!("Running");
    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
        use gstreamer::MessageView;

        match msg.view() {

            MessageView::Eos(..) => {println!("breaking"); break},
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );

//                pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "error");
                break;
            }
            MessageView::StateChanged(s) =>
            {
                    /*println!(
                    "State changed from {:?}: {:?} -> {:?} ({:?})",
                    s.get_src().map(|s| s.get_path_string()),
                    s.get_old(),
                    s.get_current(),
                    s.get_pending()
                    );*/

//                pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), format!("{:?}_{:?}", s.get_old(), s.get_current()));
                /*if state_changed.get_current() == gstreamer::State::Playing
                {
                    // Generate a dot graph f the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                    //println!("Wrote playing state!");
                }
                else if state_changed.get_current() == gstreamer::State::Paused
                {
                    // Generate a dot graph f the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                    //println!("Wrote paused state!");
                    pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "PAUSE");
                }*/

            }

            _ => ()
        }
    }

    pipeline.set_state(gstreamer::State::Null)?;
    
    return Ok(());
}

pub fn generate_encoding_profile(source_uri : &str, output_file : &str) -> DonBotResult<()>{
    ges::init()?;
    let discovered_file = gst_pbutils::Discoverer::new(gst::ClockTime::from_seconds(2))?.discover_uri(&format!("file://{}", source_uri))?;
    let profile : gst_pbutils::EncodingProfile  = gst_pbutils::EncodingProfile::from_discoverer(&discovered_file)?;

    let encoding_target = gst_pbutils::EncodingTarget::new("donbot-encoding-target", "donbot", "", &[profile]);
    encoding_target.save_to_file(output_file)?;

    return Ok(());
}
