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

static mut POST_BUILD: Option<Child> = None;
//Automatically updates and schedules the auto_stitcher binary
#[derive(Deserialize, Debug)]
pub struct PushPayload {
    pub commits : Vec<PayloadCommit>,
    pub repository : PayloadRepo,
    pub pusher : PayloadPusher
}

#[derive(Deserialize, Debug)]
pub struct PayloadCommit {
    pub id: String
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

#[get("/")]
fn ping() -> String { "pong".to_string() }


#[post("/", format = "json",data = "<payload_data>")]
fn on_push(payload_data: Json<PushPayload>) -> String{
    let payload = payload_data.into_inner();

    //DEBUG
    println!("Payload: {:?}", payload);

    if payload.pusher.name == "Mimerme" {
        println!("DonBot-rs new patch received and verified.");
        println!("New commits: ");
        for commit in payload.commits {
            println!("Commit: {}", commit.id);
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

    return "success".to_string();
}

#[test]
fn test_rebuild(){
    let cfg = Ini::load_from_file("config.ini").unwrap();
    let updater_section = cfg.section(Some("updater")).unwrap();
    let repo_path = updater_section.get("REPO_PATH").unwrap();;
    let remote = updater_section.get("REMOTE").unwrap();;
    let branch = updater_section.get("BRANCH").unwrap();;
    let post_build = updater_section.get("POST_BUILD").unwrap();

    rebuild_auto_stitcher(repo_path, remote, branch, post_build);
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

    println!("!!! Running post build script: {}", post_build);

    
    //NOTE: Post build stirng supports passing in arguments
    let args = post_build.split(" ").collect::<Vec<&str>>();

    //Run script/binary after the build to fix things up depending on the user's environment
    let mut post_build_command = Command::new(args[0]);
    for x in 1..args.len() {
        post_build_command.arg(args[x]);
    }

    unsafe {
        POST_BUILD = Some(post_build_command.spawn().unwrap());
        let post_build_proc : &mut Child = POST_BUILD.as_mut().unwrap();

        match post_build_proc.try_wait() {
            Ok(Some(status)) => println!("exited with: {}", status),
            Ok(None) => {
                let res = post_build_proc.wait();
                println!("result: {:?}", res);
            }
            Err(e) => println!("error attempting to wait: {}", e)
        }
    }
}


#[catch(422)]
fn not_found() -> String {
    return "oopsie".to_string();
}

fn main(){
    println!("Starting GitHub Webhook Server...");
    rocket::ignite().mount("/", routes![ping, on_push]).launch();
    //POST_BUILD.drop();
}
