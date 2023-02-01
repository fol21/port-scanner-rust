
pub mod tcp {
    use std::net::{ToSocketAddrs, SocketAddr, TcpListener};
    
    pub fn ip_port_is_available<A: ToSocketAddrs>(addr: A) -> bool {
        let _m = match TcpListener::bind(addr) {
            Ok(_) => true,
            Err(_) => false,
        };
        return _m;
    }

    pub fn port_is_available(port: u16) -> bool {
        let _m = match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))) {
            Ok(_) => true,
            Err(_) => false,
        };
        return _m;
    }
}