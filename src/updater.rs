mod don_bot;
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use std::process::Command;
use ini::Ini;


#[post("/")]
fn on_push(){
    println!("Processing payload...");
    println!("DonBot-rs new patch received. Starting update.");
}


fn main(){
    println!("Starting GitHub Webhook Server...);

}
