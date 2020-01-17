extern crate gstreamer as gst;
use gstreamer::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;
use std::ops::Deref;

extern crate gstreamer_editing_services_sys;
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

    //Load the encoding settings for encoding the final render
    let profiles = gst_pbutils::EncodingTarget::load_from_file(&encoding_profile_file)?.get_profiles();
    let container_profile = &profiles[0];
    let encoding_container_profile = container_profile.clone().downcast::<gst_pbutils::EncodingContainerProfile>().unwrap();
    //video_res is an arrray [width, height]
    let video_res = get_video_res(&encoding_container_profile.get_profiles())?;


    // Begin by creating a timeline with audio and video tracks
    let timeline = ges::Timeline::new_audio_video();
    // Create a new layer that will contain our timed clips.
    let layer = timeline.append_layer();
    let test_layer = timeline.append_layer();
    //Create a pipeline with the timeline
    let pipeline = ges::Pipeline::new();
    pipeline.set_timeline(&timeline)?;

    let mut current_time = gst::ClockTime::from_seconds(0);
    for clip in clips {
       let clip = ges::UriClip::new(&format!("file://{}", clip))?;

       // Apply bins onto clips using effects
       let scale = ges::Effect::new("videoscale").unwrap();
       let filter = ges::Effect::new(&format!("capsfilter caps=video/x-raw,width={},height={}",video_res[0], video_res[1])).unwrap();

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

       // NOTE: effects are added in reverse order.
       // So in this case scale comes before filter in the pipeline
       clip.add(&filter).unwrap();
       clip.add(&scale).unwrap();

       layer.add_clip(&clip)?;
    }


    println!("Reading Encoding Target From: {}", &encoding_profile_file);
    pipeline.set_render_settings(&format!("file://{}", output_file), container_profile);

    //Begin the render proccess
    pipeline.set_mode(ges::PipelineFlags::SMART_RENDER);
    run_pipeline(pipeline)?;

    return Ok(&Path::new(output_file));
}

fn run_pipeline(pipeline : ges::Pipeline) -> DonBotResult<()> {
    // Below is code to look over the pipeline bus.
    // Looks leik its mainly for debugging purposes but idk I just ripped it from here:
    // https://github.com/sdroege/gstreamer-rs/blob/be3c378f289683e8c0e7b7cfaff5dc74972bb074/examples/src/bin/playbin.rs    

    let pipeline = pipeline.upcast::<gst::Pipeline>();
    pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "NOT_PLAYING");
    
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

                pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "error");
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

                pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), format!("{:?}_{:?}", s.get_old(), s.get_current()));
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

fn get_video_res (profiles : &Vec<gst_pbutils::EncodingProfile>) -> DonBotResult<[i32; 2]> {
    for profile in profiles{
        let test = profile.get_description();
        let cap = profile.get_format();
        let cap_structure = cap.get_structure(0).unwrap();
        if (cap_structure.get_name().starts_with("video")) {
            let height = cap_structure.get_some::<i32>("height").unwrap();
            let width = cap_structure.get_some::<i32>("width").unwrap();
            return Ok([width, height]);
        }

    }

    return Err(Box::new(DBError::new("Couldn't find video resolution")));;
}

