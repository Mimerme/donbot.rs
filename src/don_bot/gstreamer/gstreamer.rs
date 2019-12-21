use gstreamer;

// TODO: Clean up the code for compilation
// Fix module naming conflicts
// 12/20/2019 ~Andros

pub fn stitch_videos() ->  Result<(), Error> {
    gstreamer::init()?

    //TODO: Check configuration here
    let pipeline = gstreamer::Pipeline::new(None);
    let filesrc = gstreamer::ElementFactory()::make("filesrc", None).map_err(|_| MissingElement("filesrc"))?;
    let filesrc2 = gstreamer::ElementFactory()::make("filesrc", None).map_err(|_| MissingElement("filesrc"))?;
    let filesink = gstreamer::ElementFactory()::make("filesink", None).map_err(|_| MissingElement("filesink"))?;
    let concat = gstreamer::ElementFactory()::make("concat", None).map_err(|_| MissingElement("concat"))?;

    filesrc.set_property("location", "/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4")
        .except("setting location property failed");
    filesrc2.set_property("location", "/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4")
        .except("setting location property failed");

    // Automatically link concat to the filesink
    concat.link(&filesink)?;

    // Manually link filesrc and filesrc2
    /*for x in clips.len() {
        let src_pad = concat.get_request_pad(format!("src_{}", x.to_string()));
        let filesrc = gstreamer::ElementFactory()::make("filesrc", 
                                                        Some(format!("clip_{}", x.to_string())));
            .map_err(|_| MissingElement("filesrc"))?;w
    }
    */

    pipeline.set_state(gstreamer::State::Playing)?;


    // Bellow is code to look over the pipeline bus.
    // Looks leik its mainly for debugging purposes but idk I just ripped it from here:
    // https://github.com/sdroege/gstreamer-rs/blob/master/examples/src/bin/decodebin.rs
    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    // This code iterates over all messages that are sent across our pipeline's bus.
    // In the callback ("pad-added" on the decodebin), we sent better error information
    // using a bus message. This is the position where we get those messages and log
    // the contained information.
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;

                {
                    match err.get_details() {
                        // This bus-message of type error contained our custom error-details struct
                        // that we sent in the pad-added callback above. So we unpack it and log
                        // the detailed error information here. details contains a glib::SendValue.
                        // The unpacked error is the converted to a Result::Err, stopping the
                        // application's execution.
                        Some(details) if details.get_name() == "error-details" => details
                            .get::<&ErrorValue>("error")
                            .unwrap()
                            .and_then(|v| v.0.lock().unwrap().take())
                            .map(Result::Err)
                            .expect("error-details message without actual error"),
                        _ => Err(ErrorMessage {
                            src: msg
                                .get_src()
                                .map(|s| String::from(s.get_path_string()))
                                .unwrap_or_else(|| String::from("None")),
                            error: err.get_error().description().into(),
                            debug: Some(err.get_debug().unwrap().to_string()),
                            cause: err.get_error(),
                        }
                        .into()),
                    }?;
                }
                {
                    return Err(ErrorMessage {
                        src: msg
                            .get_src()
                            .map(|s| String::from(s.get_path_string()))
                            .unwrap_or_else(|| String::from("None")),
                        error: err.get_error().description().into(),
                        debug: Some(err.get_debug().unwrap().to_string()),
                        cause: err.get_error(),
                    }
                    .into());
                }
            }
            MessageView::StateChanged(s) => {
                println!(
                    "State changed from {:?}: {:?} -> {:?} ({:?})",
                    s.get_src().map(|s| s.get_path_string()),
                    s.get_old(),
                    s.get_current(),
                    s.get_pending()
                );
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())

}
