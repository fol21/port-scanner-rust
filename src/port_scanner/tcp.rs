
use colored::*;
use std::net::{ToSocketAddrs, SocketAddr, TcpListener, TcpStream};

use super::state::PortState;

pub fn ip_port_is(addr: SocketAddr) -> PortState {
    let _l = match TcpListener::bind(&addr) {
        Ok(_) => true,
        Err(_) => false,
    };
    let _s = match TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(100)) {
        Ok(_) => true,
        Err(_) => false,
    };
    return PortState::match_state(_l, _s);
}

pub fn port_is_available(port: u16) -> bool {
    let _l = match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))) {
        Ok(_) => true,
        Err(_) => false,
    };
    let _s = match TcpStream::connect_timeout(&SocketAddr::from(([127, 0, 0, 1], port)), std::time::Duration::from_millis(100)) {
        Ok(_) => true,
        Err(_) => false,
    };
    return _l || _s;
}

pub fn resolve_results(addr: &String) {
    match addr.parse::<SocketAddr>() {
        Ok(_a) => {
            _print_resolved_results(_a);
            return;
        },
        Err(_) => ()
    };
    match addr.to_socket_addrs() {
        Ok(_a) => {
            println!("[{}]:", addr.bold().italic());   
            for a in _a.filter(|a| a.is_ipv4()) {
                _print_resolved_results(a);
            }
            println!("[{}]", "end".bold().italic())
        },
        Err(_) => ()
    }
}

fn _print_resolved_results(addr: SocketAddr) {
    let (ip, port) = (addr.ip(), addr.port());
    let state = match ip_port_is(addr) {
        PortState::Available => "available".green().bold(),
        PortState::Bound => "bound".cyan().bold(),
        PortState::Closed => "closed".red().bold(),
        PortState::Uknown => "uknown".red().bold(),
    };
    println!("{}:{} is {}",
        ip.to_string(),
        port,
        state
    );
}
