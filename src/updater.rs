#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod don_bot;
use ini::Ini;
use serde::{Deserialize};
use rocket_contrib::json::{Json, JsonValue};
use git2::{Repository, ObjectType, Commit};
use chrono::prelude::*;
use std::process::{Child, Command};
// For making it all thread safe
use std::sync::Mutex;

//Automatically updates and schedules the auto_stitcher binary
#[derive(Deserialize, Debug)]
pub struct PushPayload {
    pub commits : Vec<String>,
    pub repository : PayloadRepo,
    pub pusher : PayloadPusher
}


#[derive(Deserialize, Debug)]
pub struct PayloadRepo {
    pub id : u32,
    pub node_id : String,
    pub name : String,
    pub full_name : String,
    pub private : bool
}

#[derive(Deserialize, Debug)]
pub struct PayloadPusher {
    pub name : String,
    pub email : String
}

/*#[get("/". format = "json")]
fn ping(){
    res.

}*/


#[post("/", format = "json",data = "<payload_data>")]
fn on_push(payload_data: Json<PushPayload>){
    println!("Processing payload...");
    let payload = payload_data.into_inner();

    //DEBUG
    println!("Payload: {:?}", payload);

    if payload.pusher.name == "Mimerme" {
        println!("DonBot-rs new patch received and verified.");
        println!("New commits: ");
        for commit in payload.commits {
            println!("Commit: {}", commit);
        }
        //Get the latest config values
        let cfg = Ini::load_from_file("config.ini").unwrap();
        let updater_section = cfg.section(Some("updater")).unwrap();
        let repo_path = updater_section.get("REPO_PATH").unwrap();;
        let remote = updater_section.get("REMOTE").unwrap();;
        let branch = updater_section.get("BRANCH").unwrap();;
        let post_build = updater_section.get("POST_BUILD").unwrap();


        rebuild_auto_stitcher(repo_path, remote, branch, post_build);
    }
}

//TODO: Really need to figure out how to do proper error handling here. Layz tn, but def l8er
//Merging code ripped from:
//https://stackoverflow.com/questions/54100789/how-is-git-pull-done-with-the-git2-rs-rust-crate
fn find_last_commit(repo: &Repository) -> Result<Commit, String> {
     let obj = repo.head().unwrap().resolve().unwrap().peel(ObjectType::Commit).unwrap();
     match obj.into_commit() {
         Ok(c) => Ok(c),
         _ => Err("failed to find last commit from HEAD".to_string()),
     }
} 

fn rebuild_auto_stitcher(path : &str, remote : &str, branch : &str, post_build : &str) {
    println!("!!! Pulling latest updates @ {}", Local::now().to_rfc3339());
   
    //Initializing the repository and fetching the latest commits
    let repo = Repository::init(path).unwrap();
    let mut remote = repo.find_remote(remote).unwrap();
    remote.fetch(&[branch], None, None);

    let our_commit = find_last_commit(&repo).unwrap();
    let reference = repo.find_reference("FETCH_HEAD").unwrap();
    let their_commit = reference.peel_to_commit().unwrap();
    let _index = repo
                .merge_commits(&our_commit, &their_commit, None);

    println!("!!! Starting new build @ {}", Local::now().to_rfc3339());

    //Change the working directory the repository and build the project
    let path = std::path::Path::new(path);
    std::env::set_current_dir(&path).is_ok();
    let mut cargo_proc = Command::new("cargo").arg("build").spawn();
        
    cargo_proc.unwrap().wait();

    println!("!!! Running post build script @ {}", Local::now().to_rfc3339());

    //Run script/binary after the build to fix things up depending on the user's environment
    let mut post_build_command = Command::new(post_build);

    //NOTE: Post build stirng supports passing in arguments
    let args = post_build.split(" ").collect::<Vec<&str>>();

    for arg in args {
        post_build_command.arg(arg);
    }

    let post_build_proc = post_build_command.spawn();
    post_build_proc.unwrap().wait();
}




fn main(){
    println!("Starting GitHub Webhook Server...");
}
