#[test]
fn test_generate_overlay(){}

#[test]
fn test_concat_clips(){
    use super::*;

    let pipeline = stitch_videos_pipeline(vec!["/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4".to_string(),
                "/home/mimerme/Downloads/SampleVideo_1280x720_1mb.mp4".to_string()], "/home/mimerme/projects/donbot.rs/downloads/output.mp4".to_string(), 60, 44100).unwrap();
   
    //NOTE: many errors won't arise in the pipeline until we run it
    run_pipeline(pipeline);

}


#[test]
fn test_benchmark(){}
