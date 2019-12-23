use gstreamer;
use gstreamer::prelude::*;
use gstreamer::ErrorMessage;

//TODO: Check to see if we're allowed to initialize multiple times
pub fn create_pipeline(name : &str) -> Result<gstreamer::Pipeline, String> {
    gstreamer::init().map_err(|_| "gstreamer initialization failed")?;
    return Ok(gstreamer::Pipeline::new(Some(name)));
}


pub fn run_pipeline(pipeline : gstreamer::Pipeline) -> Result<(), String> {
    // Bellow is code to look over the pipeline bus.
    // Looks leik its mainly for debugging purposes but idk I just ripped it from here:
    // https://github.com/sdroege/gstreamer-rs/blob/be3c378f289683e8c0e7b7cfaff5dc74972bb074/examples/src/bin/playbin.rs    

    //TODO: Proper error handling
    pipeline.set_state(gstreamer::State::Playing).map_err(|_| "error playing")?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
        use gstreamer::MessageView;

        match msg.view() {

            MessageView::Eos(_) | MessageView::AsyncDone(_) => break,
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
                if state_changed.get_current() == gstreamer::State::Playing
                {
                    // Generate a dot graph f the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                    pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "PLAYING");
                }
            }

            _ => ()
        }
    }

    pipeline.set_state(gstreamer::State::Null).map_err(|_| "error playing")?;
    
    return Ok(());
}

pub fn create_filesrcs(pipeline : gstreamer::Pipeline, paths : Vec<String>) -> Result<Vec<gstreamer::Element>, String> {
    let mut ret = Vec::new();

    for path in paths {
        let filesrc = gstreamer::ElementFactory::make("filesrc", None).map_err(|_| "missing filesrc element")?;
        filesrc.set_property("location", &path)
            .map_err(|_| "setting location property failed")?;
        pipeline.add(&filesrc);
        ret.push(filesrc);
    }

    return Ok(ret);
}


pub fn stitch_videos(pipeline : gstreamer::Pipeline, filesrcs : Vec<gstreamer::Element>) ->  Result<gstreamer::Element, String> {
    let concat = gstreamer::ElementFactory::make("concat", None).map_err(|_| "missing concat element")?;
    
    pipeline.add(&concat);
    //pipeline.add_many(&[&filesrc, &concat, &filesink]);
    //concat.link(&filesink).map_err(|_| "problem linking concat and filesink")?;

    // Manually link filesrc and filesrc2
    for x in 0..filesrcs.len() {
        //let concat_src_pad = concat.get_request_pad(format!("src_{}", x.to_string()))?;
        //let filesrc_src_pad = filesrcs[x].get_static_pad("src_0")?;
        filesrcs[x].link_pads(Some("src_0"), 
                              &concat,
                              Some(&format!("src_{}", x.to_string())));
    }


    return Ok(concat);
}

pub fn output_video(pipeline : gstreamer::Pipeline, output_path : String, prev_elem : gstreamer::Element) -> Result<gstreamer::Element, String> {
    let filesink = gstreamer::ElementFactory::make("filesink", None).map_err(|_| "missing filesink element")?;
    filesink.set_property("location", &output_path);
    
    prev_elem.link(&filesink);

    return Ok(filesink);
}



