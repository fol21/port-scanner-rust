pub mod port_scanner;

use std::{net::{SocketAddr, SocketAddrV4}, str::FromStr};

use port_scanner::tcp::*;

fn main() {
    let addr: SocketAddr = "127.0.0.1:80".parse().expect("Unable to parse SocketAddr");
    let (ip, port) = (addr.ip(), addr.port());
    
    if ip_port_is_available(addr) {
        println!("Port {} is available", port);
    } else {
        println!("Port {} is not available", port);
    }
}
