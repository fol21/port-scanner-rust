
use colored::*;
use regex::Regex;
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

fn _process_http_www_input(input: &str, gname: &str) -> Option<String> {
    let _pattern = format!(r"^(?P<protocol>https?://)?(?:www\.)?(?P<{}>[^/?:]+)(?:(?P<port>:\d{{1,}})|(?:.*))?$", gname);
    let re = Regex::new(_pattern.as_str()).unwrap();
    let (_d, _p, _h) = match re.captures(input) {
        Some(caps) => (
            Some(caps.name(gname).unwrap().as_str().to_string()),
            match caps.name("port") {
                Some(port) => Some(port.as_str().to_string()),
                None => None,
            },
            match caps.name("protocol") {
                Some(protocol) => Some(protocol.as_str().to_string()),
                None => None,
            }
        ),
        None => (None, None, None),
    };
    return if _p.is_some() {
        Some(format!("{}{}", _d.unwrap(), _p.unwrap()))
    } else {
        match _h {
            Some(_h) => Some(format!("{}{}", _d.unwrap(), match _h.as_str() {
                "https://" => ":443",
                "http://" => ":80",
                _ => ":80",
            })),
            None => Some(format!("{}:80", _d.unwrap()))
        }
    };
}



pub enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

pub fn process_input(input: &str) -> StringOrVec {
    let resolved: StringOrVec;
    let _domain =  match _process_port_array(input) {
        Some(_d) => StringOrVec::Vec(_d),
        None => StringOrVec::String(String::from(input))
    };
    match _domain {
        StringOrVec::Vec(_addrs) => resolved = StringOrVec::Vec(
            _addrs
                .iter()
                .map(|_a| _process_http_www_input(_a.as_str(), "domain").unwrap_or(String::from("")))
                .filter(|s| s.len() > 0)
                .collect()
            ),
        StringOrVec::String(_addr) => resolved = StringOrVec::String(_process_http_www_input(_addr.as_str(), "domain").unwrap_or(_addr))
    }
    return resolved;
}

fn _process_port_array(input: &str) -> Option<Vec<String>> {
    let _pattern = r"^(?P<domain>.*):\[(?P<start>\d{1,}):(?:(?P<step>\d{1,})?:)?(?P<end>\d{1,})\]$";
    let re = Regex::new(_pattern).unwrap();
    let limits = match re.captures(input) {
        Some(caps) => Some((
            caps.name("domain").unwrap().as_str().to_string(),
            caps.name("start").unwrap().as_str().to_string().parse::<u32>().unwrap(),
            match caps.name("step") {
                Some(step) => step.as_str().to_string().parse::<u32>().unwrap(),
                None => 1,
            },
            caps.name("end").unwrap().as_str().to_string().parse::<u32>().unwrap()
        )),
        None => None,
    };
    let mut addr_list = Vec::new();
    match limits {
        Some((domain, start, step, end)) => {
            for i in (start..end).step_by(step as usize) {
                addr_list.push(format!("{}:{}", domain, i));
            }
        },
        None => (),
    }
    return if addr_list.len() > 0 {
        Some(addr_list)
    } else {
        None
    };
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

#[cfg(test)]
mod tests {
    use super::*;
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
    fn parse_input_test() {
        let mut input = "https://google.com";
        let domain = process_input(input);
        match domain {
            StringOrVec::String(domain) => assert_eq!(domain, "google.com:443"),
            StringOrVec::Vec(_) => panic!("Expected String, got Vec"),
        };
        input = "https://www.google.com:[8000::9000]";
        let domain = process_input(input);
        match domain {
            StringOrVec::Vec(_addrs) => {
                assert_eq!(_addrs.len(), 1000);
                assert_eq!(_addrs[0], "google.com:8000");
                assert_eq!(_addrs[_addrs.len()-1], "google.com:8999");
            },
            StringOrVec::String(_) => panic!("Expected Vec, got String"), 
        };
        input = "https://www.google.com:[8000:2:8007]";
        let domain = process_input(input);
        match domain {
            StringOrVec::Vec(_addrs) => {
                assert_eq!(_addrs.len(), 4);
                assert_eq!(_addrs[0], "google.com:8000");
                assert_eq!(_addrs[_addrs.len()-1], "google.com:8006");
            },
            StringOrVec::String(_) => panic!("Expected Vec, got String"), 
        };
        input = "127.0.0.1:80";
        let domain = process_input(input);
        match domain {
            StringOrVec::String(domain) => assert_eq!(domain, "127.0.0.1:80"),
            StringOrVec::Vec(_) => panic!("Expected String, got Vec"),
        };
    }
}
