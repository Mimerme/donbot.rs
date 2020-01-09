extern crate gstreamer as g;
use gstreamer::prelude::*;
use gstreamer::ErrorMessage;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

extern crate gstreamer_pbutils as gst_pbutils;
use gst_pbutils::prelude::*;

extern crate glib;


pub fn stitch_videos_pipeline(clips : Vec<String>, output : String, fps : i8, sample_rate : u32) -> Result<ges::Pipeline, String>{
    ges::init().unwrap();

    // Begin by creating a timeline with audio and video tracks
    let timeline = ges::Timeline::new_audio_video();
    // Create a new layer that will contain our timed clips.
    let layer = timeline.append_layer();
    let pipeline = ges::Pipeline::new();
    pipeline.set_timeline(&timeline).unwrap();

    let mut current_time = g::ClockTime::from_seconds(0);
    for clip in clips {
        println!("file://{}", clip);
        let clip = ges::UriClip::new(&format!("file://{}", clip)).expect("Failed to create clip");

       // Add an effect to the clip's video stream.
       //let effect = ges::Effect::new("agingtv").expect("Failed to create effect");
       //clip.add(&effect).unwrap();

       /*println!(
           "Agingtv scratch-lines: {}",
           clip.get_child_property("scratch-lines")
               .unwrap()
               .serialize()
               .unwrap()
       );*/

       // Retrieve the asset that was automatically used behind the scenes, to
       // extract the clip from.
       let asset = clip.get_asset().unwrap();
       let duration = asset
           .downcast::<ges::UriClipAsset>()
           .unwrap()
           .get_duration();
       /*println!(
           "Clip duration: {} - playing file from {} for {}",
           duration,
           current_time,
           duration
       );*/

       // The inpoint specifies where in the clip we start, the duration specifies
       // how much we play from that point onwards. Setting the inpoint to something else
       // than 0, or the duration something smaller than the clip's actual duration will
       // cut the clip.
       clip.set_inpoint(g::ClockTime::from_seconds(0));
       clip.set_start(current_time);
       clip.set_duration(duration);
       current_time = current_time + duration; 
       layer.add_clip(&clip).unwrap();
    }

    let container_cap = g::Caps::new_simple("video/quicktime", &[("variant", &"iso")]);      
    let audio_cap = g::Caps::new_simple("audio/mpeg", &[
                                        ("mpegversion", &4), 
                                        ("rate", &44100),
                                        ("channels", &2),
    ]);

    let video_cap = g::Caps::new_simple("video/x-h264", &[
                                        ("width",&1280),
                                        ("height", &720),
                                        ("framerate",&g::Fraction::new(60, 1)),
    ]);

    let a = gst_pbutils::Discoverer::new(g::ClockTime::from_seconds(2)).unwrap().discover_uri("file:///home/mimerme/projects/donbot.rs/downloads/test/TardyPlayfulMangoImGlitch.mp4").unwrap();
    //let profile : gst_pbutils::EncodingContainerProfile  = gst_pbutils::EncodingProfile::from_discoverer(&a).unwrap().downcast::<gst_pbutils::EncodingContainerProfile>().unwrap();
    //let profile : gst_pbutils::EncodingProfile  = gst_pbutils::EncodingProfile::from_discoverer(&a).unwrap();

    //println!("Encoding profile name: {}", profile.get_name().unwrap());
    //println!("Encoding profile format: {:?}", profile.get_format());
    //println!("Encoding profile description: {}", profile.get_description().unwrap());
    //println!("Encoding profile presence: {}", profile.get_presence());
    //println!("f: {:?}", profile);
    let empty_cap = g::Caps::new_empty();

    let mut prof_container_builder = gst_pbutils::EncodingContainerProfileBuilder::new();
    prof_container_builder = prof_container_builder.name("Quicktime");
    prof_container_builder = prof_container_builder.format(&container_cap);
    prof_container_builder = prof_container_builder.description("DonBot's encoding profile for Quicktime");

    
    let mut prof_audio_builder = gst_pbutils::EncodingAudioProfileBuilder::new();
    prof_audio_builder = prof_audio_builder.format(&audio_cap);
    prof_audio_builder = prof_audio_builder.allow_dynamic_output(true);
    prof_audio_builder = prof_audio_builder.enabled(true);
    prof_audio_builder = prof_audio_builder.restriction(&empty_cap);

    let mut prof_video_builder = gst_pbutils::EncodingVideoProfileBuilder::new();
    prof_video_builder = prof_video_builder.format(&video_cap);
    prof_video_builder = prof_video_builder.allow_dynamic_output(true);
    prof_video_builder = prof_video_builder.enabled(true);
    prof_video_builder = prof_video_builder.restriction(&empty_cap);

    let video_profile = prof_video_builder.build().unwrap();
    let audio_profile = prof_audio_builder.build().unwrap();

    prof_container_builder = prof_container_builder.add_profile(&audio_profile);
    prof_container_builder = prof_container_builder.add_profile(&video_profile);

    let container_profile : gst_pbutils::EncodingContainerProfile = prof_container_builder.build().unwrap();
    
    // let profile = container_profile.upcast::<gst_pbutils::EncodingProfile>();
    //let test_encoding_target = gst_pbutils::EncodingTarget::new("test2", "donbot", "a test", &[]);
    let profiles = gst_pbutils::EncodingTarget::load_from_file("/home/mimerme/test1").unwrap().get_profiles();


    timeline.save_to_uri::<ges::UriClipAsset>(&format!("file://{}", "/home/mimerme/lul.mp4"), None, true);
    pipeline.set_render_settings("file:///home/mimerme/f.mp4", &profiles[0]);
    pipeline.set_mode(ges::PipelineFlags::RENDER);

    // Load a clip from the given uri and add it to the layer.

    pipeline
        .set_state(g::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(g::CLOCK_TIME_NONE) {
        use g::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(g::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    Ok(pipeline)
}



pub fn run_pipeline(pipeline : ges::Pipeline) -> Result<(), String> {
    // Below is code to look over the pipeline bus.
    // Looks leik its mainly for debugging purposes but idk I just ripped it from here:
    // https://github.com/sdroege/gstreamer-rs/blob/be3c378f289683e8c0e7b7cfaff5dc74972bb074/examples/src/bin/playbin.rs    

    //pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "NOT_PLAYING");
    
    //TODO: Proper error handling
    pipeline.set_state(gstreamer::State::Playing).map_err(|_| "error playing").unwrap();

    println!("Running");

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
        use gstreamer::MessageView;

        //println!("Message: {:?}", msg.view());

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

    pipeline.set_state(gstreamer::State::Null).map_err(|_| "error playing")?;
    
    return Ok(());
}

