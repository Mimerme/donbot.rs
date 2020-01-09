extern crate gstreamer as g;
use gstreamer::prelude::*;
use gstreamer::ErrorMessage;
use ini::Ini;

//based on this pipeline
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
pub fn stitch_videos(clips : Vec<String>, cfg : Ini) -> Result<g::Pipeline, String>{
    g::init().map_err(|_| "gstreamer initialization failed")?;

    println!("Constructing pipeline");

    let pipeline = g::Pipeline::new(None);
   
    let progress_report = g::ElementFactory::make("progressreport", None).map_err(|_| "missing progressreport element")?;
    let filesink = g::ElementFactory::make("filesink", None).map_err(|_| "missing filesink element")?;
    let video_concat = g::ElementFactory::make("concat", None).map_err(|_| "missing concat element")?;
    let audio_concat = g::ElementFactory::make("concat", None).map_err(|_| "missing concat element")?;
   

    //converters
    let audioconvert = g::ElementFactory::make("audioconvert", None).unwrap();
    let audioresample = g::ElementFactory::make("audioresample", None).unwrap();
    let videoscale = g::ElementFactory::make("videoscale", None).unwrap();
    let videoconvert = g::ElementFactory::make("videoconvert", None).unwrap();
    let videorate = g::ElementFactory::make("videorate", None).unwrap();

    //Encoders, parser, and muxer
    let x264enc = g::ElementFactory::make("x264enc", None).map_err(|_| "missing x264enc element")?;
    let faac = g::ElementFactory::make("faac", None).map_err(|_| "missing faac element")?;
    let aacparse = g::ElementFactory::make("aacparse", None).map_err(|_| "missing aacparse element")?;
    let muxer = g::ElementFactory::make("mp4mux", None).map_err(|_| "missing mpegtsmux element")?;


    println!("Elements initialized");

    x264enc.set_property("tune", &4.to_value());
    //filesink.set_property("sync", &true.to_);
    //Setting the properties of the elements
    filesink.set_property("location", &output);
    progress_report.set_property("format", &"seconds");


    //Adding the encoding portion of the pipeline here
    pipeline.add_many(&[&x264enc, &filesink, &muxer, &audio_concat, &video_concat, &faac, &aacparse, &audioresample, &progress_report, &videoconvert, &videorate, &audioconvert, &videoscale]);

    let video_queue = g::ElementFactory::make("queue", None).map_err(|_| "missing queue element").unwrap();
    let audio_queue = g::ElementFactory::make("queue", None).map_err(|_| "missing queue element").unwrap();
    pipeline.add_many(&[&video_queue, &audio_queue]);
    

    //Linking the video stream concatnation
    let video_stream = [&video_concat, &videoconvert, &videorate, &x264enc, &video_queue, &muxer];
    g::Element::link_many(&video_stream);

    //Linking the audio stream concatnation
    let audio_stream = [&audio_concat, &audioresample, &audioconvert, &faac, &aacparse, &audio_queue, &muxer];
    g::Element::link_many(&audio_stream);

    //Linking the fileoutput of the pipline and a progress report
    let muxer_to_file = [&muxer, &filesink];
    g::Element::link_many(&muxer_to_file);

    let mut count = -1;
    for clip in clips {
        count = count + 1;

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
                    panic!("Failed to get media type from pad");

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
            pipeline.add_many(&[&capsfilter]);
            capsfilter.set_property("caps", &video_cap.to_value()).unwrap();

            let caps_sink = capsfilter.get_static_pad("sink").unwrap();

            //capsfilter.link(&queue);

            let queue_sink = queue.get_static_pad("sink").unwrap();
            let queue_src = queue.get_static_pad("src").unwrap();
            src_pad.link(&queue_sink);
            let concat_sink = video_concat.get_request_pad(&format!("sink_{}", count)).unwrap();
            queue_src.link(&concat_sink);
            //queue.link(&video_concat);

            capsfilter.sync_state_with_parent();
            //println!("Added video pad");
        }
        else if is_audio {
            //Resample, interleave, and flatten the audio
            //NOTE: Might want to change how the audio is processed here
            //TODO: wat 
            //Create the caps
            let resample_cap = if 1 == 0 { 
                g::Caps::new_simple("audio/x-raw", &[])           
            }
            else {
                g::Caps::new_simple("audio/x-raw", &[("rate", &44100)]) 
            };
            let channel_cap = g::Caps::new_simple("audio/x-raw", &[("channels", &2)]);

            //let channel_capfilter = g::ElementFactory::make("capsfilter", None).unwrap();
            //let resample_capfilter = g::ElementFactory::make("capsfilter", None).unwrap();
            //let audioresample =  g::ElementFactory::make("audioresample", None).map_err(|_| "missing audioresample element").unwrap();
            //let audioconverter = g::ElementFactory::make("audioconvert", None).map_err(|_| "missing audioconverter element").unwrap();

            //resample_capfilter.set_property("caps", &resample_cap.to_value()).unwrap();
            //channel_capfilter.set_property("caps", &channel_cap.to_value()).unwrap();

            //pipeline.add_many(&[&audioconverter, &audioresample, &channel_capfilter, &resample_capfilter]);

            //let resample_sink = audioresample.get_static_pad("sink").unwrap();
            //Link our dynamic pipeline to the concat pad
            //src_pad.link(&resample_sink);
            //println!("Sample Rate: {:?}", 44100);
            //TODO: Fix up the cap building
            //audioresample.link(&resample_capfilter);
            //resample_capfilter.link(&audioconverter);
            //audioresample.link_filtered(&audioconverter, Some(&resample_cap));

            //let cap_src = channel_capfilter.get_static_pad("src").unwrap();
            //println!("Requested audio sink_{}", count);
            let queue_sink = queue.get_static_pad("sink").unwrap();
            let queue_src = queue.get_static_pad("src").unwrap();
            let concat_sink = audio_concat.get_request_pad(&format!("sink_{}", count)).unwrap();

            //audioconverter.link(&channel_capfilter);
            //channel_capfilter.link(&audio_concat);
            src_pad.link(&queue_sink);
            queue_src.link(&concat_sink);
            //queue.link(&audio_concat);

            //audioresample.sync_state_with_parent();
            //audioconverter.sync_state_with_parent();
            //channel_capfilter.sync_state_with_parent();
            //resample_capfilter.sync_state_with_parent();
            //println!("Added audio pad");
        }
        else{
            panic!("wtf");
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

    pipeline.set_state(gstreamer::State::Null).map_err(|_| "error playing")?;
    
    return Ok(());
}

