pub mod port_scanner;

use std::env;
use std::fs::File;
use std::io::read_to_string;

use port_scanner::json_scanner::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("Unable to open file");
    let raw = read_to_string(f).expect("Unable to read file");

    let json = json::parse(&raw).expect("Unable to parse json");
    read_from_json(&json);
    

}
