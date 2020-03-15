// cron alternative
// TODO: probably add more configuration options


use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
use clokwerk::Interval::*;
use std::thread;
use std::time::Duration;
use ini::Ini;
use std::process::{Child, Command};

fn main(){
    let cfg = Ini::load_from_file("config.ini").unwrap();
    //Prepare the string to move without consuming ownership of cfg
    let command = String::from(cfg.section(Some("scheduler")).unwrap().get("COMMAND").unwrap());

    // Create a new scheduler
    let mut scheduler = Scheduler::new();
    // or a scheduler with a given timezone
    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    // Schedule a task every 24 hours
    scheduler.every(24.hours()).run(move || {
        println!("Running scheduled process");

        //NOTE: runs without arguments
        let mut proc = Command::new(&command).spawn().unwrap();

        match proc.try_wait() {
            Ok(Some(status)) => println!("exited with: {}", status),
            Ok(None) => {
                let res = proc.wait();
                println!("result: {:?}", res);
            }
            Err(e) => println!("error attempting to wait: {}", e)
        }


    });


    println!("Starting scheduler");


    while(true) {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(1000));
    }
//    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    // The scheduler stops when `thread_handle` is dropped, or `stop` is called
    //thread_handle.stop();
    

}
