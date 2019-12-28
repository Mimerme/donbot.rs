use ini::Ini;

#[test]
fn test_upload_video(){
    use super::*;

    let cfg = Ini::load_from_file("config.ini").unwrap();
    println!("Response: {:?}", upload_video(cfg, "test".to_string()).unwrap()); 
}
