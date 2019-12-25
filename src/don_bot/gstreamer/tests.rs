#[test]
fn test_generate_overlay(){}

#[test]
fn test_concat_clips(){
    use super::*;

    let pipeline = create_pipeline(None).unwrap();
    println!("Pipeline created");

    let mut filesrcs = create_filesrcs(&pipeline, vec!["/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4".to_string(),
                "/home/mimerme/projects/donbot.rs/downloads/o.mp4".to_string()]).unwrap();
    //let mut filesrcs = create_filesrcs(&pipeline, vec!["/home/mimerme/projects/donbot.rs/downloads/AT-cm585662124.mp4".to_string()]).unwrap();
    println!("Filesrcs created");

    //Add a callback when new pads are added
    decode_files(&pipeline, filesrcs, || {
        let concat = 


    });

    let concat = stitch_videos(&pipeline, ).unwrap();
    println!("Concat created");

    output_video(&pipeline, "/home/mimerme/projects/donbot.rs/downloads/test.mp4".to_string(), concat);
    run_pipeline(pipeline);
}

#[test]
fn test_benchmark(){}
