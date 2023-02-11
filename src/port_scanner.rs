pub mod state;
pub mod tcp;
pub mod json_scanner;

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