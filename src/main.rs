extern crate clap;
use clap::{Arg, App};

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn write_robots(data: &str, p: String) {
    let path = Path::new(&p).join("robots.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn main() {

    let opts = App::new("robots")
                    .version("0.1.0")
                    .author("vedranvinko")
                    .arg(Arg::with_name("environment")
                        .short("e")
                        .long("environment")
                        .takes_value(true)
                        .required(false)
                        .default_value("dev")
                        .value_name("environment")
                        .help("Specify an running environment"))
                    .get_matches();

    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let c = opts.value_of("environment").unwrap();

    match c.as_ref() {
        "dev" => write_robots(include_str!("dev.txt"), cwd),
        "prod" => write_robots(include_str!("prod.txt"), cwd),
        _ => return,
    }
}
