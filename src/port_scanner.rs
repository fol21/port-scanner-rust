
pub mod tcp {
    use colored::*;
    use std::net::{ToSocketAddrs, SocketAddr, TcpListener, TcpStream};
    
    pub fn ip_port_is_available(addr: SocketAddr) -> bool {
        let _l = match TcpListener::bind(&addr) {
            Ok(_) => true,
            Err(_) => false,
        };
        let _s = match TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(100)) {
            Ok(_) => true,
            Err(_) => false,
        };
        return _l || _s;
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
                for a in _a.filter(|a| a.is_ipv4()) {
                    _print_resolved_results(a);
                }
            },
            Err(_) => ()
        }
    }
    
    fn _print_resolved_results(addr: SocketAddr) {
        let (ip, port) = (addr.ip(), addr.port());
        println!("{}:{} is {}",
            ip.to_string(),
            port,
            if ip_port_is_available(addr) {"available".green().bold()} else {"not available".red().bold()}
        );
    }
}

pub mod json_scanner {
    use std::net::{SocketAddr, ToSocketAddrs};
    use colored::*;


    use super::tcp::*;

    pub fn read_from_json(json: &json::JsonValue) {
        match json {
            json::JsonValue::Object(root) => read_object(&root),
            json::JsonValue::Array(root) => read_adresses(&root),
            _ => panic!("Root is not an object or array"),
        };
    }

    pub fn read_adresses(values: &Vec<json::JsonValue>) {
        for v in values.clone() {
            match v {
                json::JsonValue::Short(addr) => {
                    resolve_results(&addr.to_string())
                },
                _ => ()
            }
        }
    }
    
    pub fn read_object(obj: &json::object::Object) {
        for (key, value) in obj.iter() {
            println!("[{}]:\n", key.bold().yellow());
            match value {
                json::JsonValue::Object(v) => read_object(v),
                json::JsonValue::Array(v) => {read_adresses(v); println!("");},
                _ => ()
            }
        }
    }    
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, ToSocketAddrs, TcpStream};

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

    #[test]
    fn bind_to_socket() {
        use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
        let listener =  match TcpListener::bind(("127.0.0.1", 6379)) {
            Ok(_) => true,
            Err(_) => false,
        };
        let stream = match TcpStream::connect(("127.0.0.1", 6379)) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(listener || stream);
    }

    #[test]
    fn read_strings_from_json() {
        let json = json::parse(r#"["google.com:80","google.com:443"]"#).expect("Unable to parse json");
        match json {
            json::JsonValue::Array(values) => {
                for v in values {
                   assert!(v.is_string());
                }
            }
            _ => panic!("Root is not an array"),
        }
    }
}