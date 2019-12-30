extern crate gstreamer as g;
use gstreamer::prelude::*;
use gstreamer::ErrorMessage;


// based on this pipeline
//gst-launch-1.0 mpegtsmux name=mux ! filesink location=lul.mp4  
//               concat name=c ! videoconvert ! videoscale ! x264enc ! mux.  
//               concat name=d ! audioconvert ! faac ! aacparse ! mux.  
//               filesrc location=~/projects/donbot.rs/downloads/AT-cm585662124.mp4 ! decodebin name=f1  
//               filesrc location=~/Downloads/SampleVideo_1280x720_1mb.mp4 ! decodebin name=f2  
//               f2. ! video/x-raw ! queue ! c.  
//               f1. ! capsfilter caps=video/x-raw ! queue ! c.  
//               f2. ! audioresample ! audio/x-raw,rate=44100 ! audioconvert ! audio/x-raw,channels=2 ! queue ! d.  
//               f1. ! audio/x-raw ! queue ! d.

// Returns a pipeline to to run
// Set sample_rate to -1 to use the default sample rate
//TODO: Probaly work on this pipeline a bit more. Doesn't even use the fps parameter LOL
pub fn stitch_videos_pipeline(clips : Vec<String>, output : String, fps : i8, sample_rate : u32) -> Result<g::Pipeline, String>{
    g::init().map_err(|_| "gstreamer initialization failed")?;

    println!("Constructing pipeline");

    let pipeline = g::Pipeline::new(None);
    
    let filesink = g::ElementFactory::make("filesink", None).map_err(|_| "missing filesink element")?;
    let video_concat = g::ElementFactory::make("concat", None).map_err(|_| "missing concat element")?;
    let audio_concat = g::ElementFactory::make("concat", None).map_err(|_| "missing concat element")?;
    
    //Encoders, parser, and muxer
    let x264enc = g::ElementFactory::make("x264enc", None).map_err(|_| "missing x264enc element")?;
    let faac = g::ElementFactory::make("faac", None).map_err(|_| "missing faac element")?;
    let aacparse = g::ElementFactory::make("aacparse", None).map_err(|_| "missing aacparse element")?;
    let audioconvert = g::ElementFactory::make("audioconvert", None).map_err(|_| "missing audioconvert element")?;
    let muxer = g::ElementFactory::make("mpegtsmux", None).map_err(|_| "missing mpegtsmux element")?;

    println!("Elements initialized");

    //Setting the properties of the elements
    filesink.set_property("location", &output);


    //Adding the encoding portion of the pipeline here
    pipeline.add_many(&[&x264enc, &filesink, &muxer, &audio_concat, &video_concat, &faac, &aacparse, &audioconvert]);

    //Linking the video stream concatnation
    let video_stream = [&video_concat, &x264enc, &muxer];
    g::Element::link_many(&video_stream);

    //Linking the audio stream concatnation
    let audio_stream = [&audio_concat, &audioconvert, &faac, &aacparse, &muxer];
    g::Element::link_many(&audio_stream);

    let muxer_to_file = [&muxer, &filesink];
    g::Element::link_many(&muxer_to_file);

    for clip in clips {
        // Prepare some weak references because decodebin will be storing references
        // to these GstObjects cause we use  them in the move closure
        let pipeline_weak = pipeline.downgrade();
        let video_concat_weak = video_concat.downgrade();
        let audio_concat_weak = audio_concat.downgrade();

        //Create a filesrcs and decodebin element for each file
        let filesrc = g::ElementFactory::make("filesrc", None).map_err(|_| "missing filesrc element")?;
        //We're using decodebin here to demux the files
        let decodebin = g::ElementFactory::make("decodebin", None).map_err(|_| "missing decodebin element")?;


        filesrc.set_property("location", &clip)
            .map_err(|_| "setting location property failed")?;

        pipeline.add_many(&[&filesrc, &decodebin]);
        
        filesrc.link(&decodebin);

        decodebin.connect_pad_added(move |_, src_pad| {

            // Convert the weak refernece back into strong ones so we can use them
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return
            };
            let video_concat = match video_concat_weak.upgrade() {
                Some(concat) => concat,
                None => return
            };
            let audio_concat = match audio_concat_weak.upgrade() {
                Some(concat) => concat,
                None => return
            };

           // Try to detect whether the raw stream decodebin provided us with
           // just now is either audio or video (or none of both, e.g. subtitles).
           let (is_audio, is_video) = {
            let media_type = src_pad.get_current_caps().and_then(|caps| {
                caps.get_structure(0).map(|s| {
                    let name = s.get_name();
                    (name.starts_with("audio/"), name.starts_with("video/"))
                })
            });

            match media_type {
                None => {
                    /*gst_element_warning!(
                        dbin,
                        gst::CoreError::Negotiation,
                        ("Failed to get media type from pad {}", src_pad.get_name())
                    );*/
                    println!("Failed to get media type from pad");

                    return;
                }
                Some(media_type) => media_type,
            }
        };


        let queue = g::ElementFactory::make("queue", None).map_err(|_| "missing queue element").unwrap();
        pipeline.add(&queue);

        if is_video {
            let capsfilter = g::ElementFactory::make("capsfilter", None).unwrap();
            let video_cap = g::Caps::new_simple("video/x-raw", &[]);
           
            pipeline.add(&capsfilter);

            capsfilter.set_property("caps", &video_cap.to_value()).unwrap();

            let caps_sink = capsfilter.get_static_pad("sink").unwrap();

            src_pad.link(&caps_sink);
            capsfilter.link(&queue);
            queue.link(&video_concat);

            capsfilter.sync_state_with_parent();
        }
        else if is_audio {
            //Resample, interleave, and flatten the audio
            //NOTE: Might want to change how the audio is processed here
            
            //Create the caps
            let resample_cap = if 1 == 0 { 
                g::Caps::new_simple("audio/x-raw", &[])           
            }
            else {
                g::Caps::new_simple("audio/x-raw", &[("rate", &"44100")]) 
            };
            let channel_cap = g::Caps::new_simple("audio/x-raw", &[("channels", &2)]);

            let audioresample =  g::ElementFactory::make("audioresample", None).map_err(|_| "missing audioresample element").unwrap();
            let audioconverter = g::ElementFactory::make("audioconvert", None).map_err(|_| "missing audioconverter element").unwrap();

            pipeline.add_many(&[&audioconverter, &audioresample]);

            let resample_sink = audioresample.get_static_pad("sink").unwrap();
            //Link our dynamic pipeline to the concat pad
            src_pad.link(&resample_sink);
            //println!("Sample Rate: {:?}", 44100);
            //TODO: Fix up the cap building
            audioresample.link_filtered(&audioconverter, Some(&g::Caps::new_simple("audio/x-raw", &[("rate", &44100)])));
            //audioresample.link_filtered(&audioconverter, Some(&resample_cap));
            audioconverter.link_filtered(&queue, Some(&channel_cap));
            queue.link(&audio_concat);

            audioresample.sync_state_with_parent();
            audioconverter.sync_state_with_parent();
        }

        queue.sync_state_with_parent();

            //Remember to synchronize your elements state to their parents 
            //if you create new elements within the closure!!!
        });
    };

    return Ok(pipeline);
}

pub fn run_pipeline(pipeline : g::Pipeline) -> Result<(), String> {
    // Below is code to look over the pipeline bus.
    // Looks leik its mainly for debugging purposes but idk I just ripped it from here:
    // https://github.com/sdroege/gstreamer-rs/blob/be3c378f289683e8c0e7b7cfaff5dc74972bb074/examples/src/bin/playbin.rs    

    pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "NOT_PLAYING");
    
    //TODO: Proper error handling
    pipeline.set_state(gstreamer::State::Playing).map_err(|_| "error playing")?;

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
                break;
            }
            MessageView::StateChanged(state_changed) =>
            {
                println!("State changed");
                if state_changed.get_current() == gstreamer::State::Playing
                {
                    // Generate a dot graph f the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                    println!("Wrote playing state!");
                    pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "PLAYING");
                }
                else if state_changed.get_current() == gstreamer::State::Paused
                {
                    // Generate a dot graph f the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                    println!("Wrote paused state!");
                    pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "PAUSE");
                }

            }

            _ => ()
        }
    }

    pipeline.set_state(gstreamer::State::Null).map_err(|_| "error playing")?;
    
    return Ok(());
}

