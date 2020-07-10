use std::{thread, time};

pub struct Config {
    work_time: time::Duration, //in seconds
    break_time: time::Duration, // in seconds
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>{
        let work_time: time::Duration = match args.next() {
            Some(arg) => time::Duration::new(arg*60,0),
            None => return Err("Please enter work time in seconds"),
        };

        let break_time: time::Duration = match args.next() {
            Some(arg) => time::Duration::new(arg.parse::<u64>().unwrap()*60,0),
            None => return Err("Please enter break time in seconds"),
        };

        Ok(Config {
            work_time,
            break_time,
        })
    }
}

pub fn run(configuration: Config) {
    println!("Work time!");
    thread::sleep(configuration.work_time);
    println!("Break time!");
    thread::sleep(configuration.break_time);
}
