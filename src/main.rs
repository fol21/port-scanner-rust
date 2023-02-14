#![feature(proc_macro_hygiene)]
pub mod port_scanner;

use std::env;
use std::fs::File;
use std::io::read_to_string;
use commander_rust::{ Cli, command, option, entry, run };

use port_scanner::json_scanner::*;

#[option(-s, --async, "Asynchronous scanning")]
#[option(-w, --workers <workers>, "Number of workers")]
#[command(scan <filepath> , "Scan a JSON file")]
fn scan(filepath: String, cli: Cli) {
    let f = File::open(&filepath).expect("Unable to open file");
    let raw = read_to_string(f).expect("Unable to read file");

    let json = json::parse(&*raw).expect("Unable to parse json");
    if cli.has("async") {
        if cli.has("workers") {
            unsafe {
                read_from_json(&json, cli.get("workers")[0].parse::<usize>().expect("Unable to parse number of threads"));
            }
        } else {
            unsafe {
                read_from_json(&json, num_cpus::get());
            }
        }
    } else {
        read_from_json_sync(&json);
    }
}

#[entry]
fn main() {  
    run!();
}
