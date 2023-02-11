use colored::*;
use regex::Regex;

use super::tcp::*;


fn _process_http_www_input(input: &str, gname: &str) -> Option<String> {
    let _pattern = format!(r"^(?:https?://)?(?:www\.)?(?P<{}>[^/?:]+)(?:(?P<port>:\d{{1,}})|(?:.*))?$", gname);
    let re = Regex::new(_pattern.as_str()).unwrap();
    let (_d, _p) = match re.captures(input) {
        Some(caps) => (
            Some(caps.name(gname).unwrap().as_str().to_string()),
            match caps.name("port") {
                Some(port) => Some(port.as_str().to_string()),
                None => None,
            }
        ),
        None => (None, None),
    };
    return if _p.is_some() {
        Some(format!("{}{}", _d.unwrap(), _p.unwrap()))
    } else {
        _d
    };
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

pub fn read_from_json_sync(json: &json::JsonValue) {
    match json {
        json::JsonValue::Object(root) => read_object_sync(&root),
        json::JsonValue::Array(root) => read_adresses(&root),
        _ => panic!("Root is not an object or array"),
    };
}

pub unsafe fn read_from_json(json: &json::JsonValue, workers: usize) {
    panic!("Not implemented yet");
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

pub fn read_object_sync(obj: &json::object::Object) {
    for (key, value) in obj.iter() {
        println!("[{}]:\n", key.bold().yellow());
        match value {
            json::JsonValue::Object(v) => read_object_sync(v),
            json::JsonValue::Array(v) => {read_adresses(v); println!("");},
            _ => ()
        }
    }
}

pub unsafe fn read_object(obj: &json::object::Object) {
    panic!("Not implemented yet");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let mut input = "https://google.com";
        let domain = process_input(input);
        match domain {
            StringOrVec::String(domain) => assert_eq!(domain, "google.com"),
            StringOrVec::Vec(_) => panic!("Expected String, got Vec"),
        };
        input = "https://google.com:[8000:9000]";
        let domain = process_input(input);
        match domain {
            StringOrVec::Vec(_addrs) => {
                assert_eq!(_addrs.len(), 1000);
                assert_eq!(_addrs[0], "google.com:8000");
                assert_eq!(_addrs[999], "google.com:8999");
            },
            StringOrVec::String(_) => panic!("Expected Vec, got String"), 
        };
    }
}