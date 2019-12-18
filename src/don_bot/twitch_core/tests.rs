#[test]
fn test_get_helix_clips(){
    use super::*;

    let res = get_helix_top_clips(reqwest::blocking::Client::new(), "29595".to_string());
    
    match res {
        Result::Ok(res) => assert!(true),
        Result::Err(_) =>  assert!(false)
    }

}

fn test_download_clip(){
    use super::*;

    let res = get_helix_top_clips(reqwest::blocking::Client::new(), "29595".to_string());
    
    match res {
        Result::Ok(res) => assert!(true),
        Result::Err(_) =>  assert!(false)
    }

}
