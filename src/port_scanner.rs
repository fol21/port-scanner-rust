
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

pub mod json_scanner {
    use std::net::{SocketAddr, ToSocketAddrs};

    use super::tcp::*;

    pub fn read_from_json(json: &json::JsonValue) {
        match json {
            json::JsonValue::Object(root) => read_object(&root),
            json::JsonValue::Array(root) => read_adresses(&root),
            _ => panic!("Root is not an object or array"),
        };
    }

    pub fn read_adresses(values: &Vec<json::JsonValue>) {
        for v in values {
            match v {
                json::JsonValue::String(addr) => {
                    match addr.parse::<SocketAddr>() {
                        Ok(_a) => {
                            let (ip, port) = (_a.ip(), _a.port());
                            
                            if ip_port_is_available(addr) {
                                println!("Port {} is available", port);
                            } else {
                                println!("Port {} is not available", port);
                            }
                        },
                        Err(_a) => {
                            match addr.to_socket_addrs() {
                                Ok(_a) => {
                                    for a in _a.filter(|a| a.is_ipv4()) {
                                        let (ip, port) = (a.ip(), a.port());
                                        
                                        if ip_port_is_available(addr) {
                                            println!("Port {} is available", port);
                                        } else {
                                            println!("Port {} is not available", port);
                                        }
                                    }
                                },
                                Err(_) => ()
                            }
                        },
                    };
                },
                _ => ()
            }
        }
    }
    
    pub fn read_object(obj: &json::object::Object) {
        for (key, value) in obj.iter() {
            match value {
                json::JsonValue::Object(v) => read_object(v),
                json::JsonValue::Array(v) => read_adresses(v),
                _ => ()
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, ToSocketAddrs};

    #[test]
    fn parse_dns_values() {
        let addr = "google.com:80".to_socket_addrs().expect("Unable to parse SocketAddr");
        for a in addr.filter(|a| a.is_ipv4()) {
            println!("{}:{}", a.ip(), a.port());
            assert!(a.is_ipv4() || a.is_ipv6());
        }
    }

    #[test]
    fn parse_socket_adrrs_values() {
        let addr = "4.4.4.4:80".parse::<SocketAddr>().expect("Unable to parse SocketAddr");
        println!("{}:{}", addr.ip(), addr.port());
        assert!(addr.is_ipv4() || addr.is_ipv6());
    }
}