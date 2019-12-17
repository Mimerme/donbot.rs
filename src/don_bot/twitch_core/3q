#[test]
fn test_clip_success(){
    use super::*;

    let res = get_helix_top_clips(reqwest::blocking::Client::new(), "0".to_string());
    
    match res {
        Result::Ok(res) => assert!(res),
        Result::Err(_) =>  assert!(false)
    }

}
