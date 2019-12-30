#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod don_bot;
use std::process::Command;
use ini::Ini;
use serde::{Deserialize};
use rocket_contrib::json::{Json, JsonValue};
use git2::Repository;

//Automatically updates and schedules the auto_stitcher binary

#[derive(Deserialize)]
pub struct PushPayload {
    pub commits : Vec<String>,
    pub repository : PayloadRepo,
    pub pusher : PayloadPusher
}


#[derive(Deserialize)]
pub struct PayloadRepo {
    pub id : u32,
    pub node_id : String,
    pub name : String,
    pub full_name : String,
    pub private : bool
}

#[derive(Deserialize)]
pub struct PayloadPusher {
    pub name : String,
    pub email : String
}

#[post("/", format = "json",data = "<payload_data>")]
fn on_push(payload_data: Json<PushPayload>){
    println!("Processing payload...");
    let payload = payload_data.into_inner();

    //DEBUG
    println!("Payload: {:?}", payload);

    if payload.pusher.name == "Mimerme" {
        println!("DonBot-rs new patch received and verified.");
        println!("New commits: ")
        for commit in payload.commits {
            println!("Commit: {}", commit);
        }
    }
}

fn git_pull() {
    let repo = match Repository::init() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e)
    };
}

fn rebuild_auto_stitcher() {

}


fn main(){
    println!("Starting GitHub Webhook Server...");

}
