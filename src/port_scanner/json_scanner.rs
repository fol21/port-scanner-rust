use colored::*;

use super::tcp::*;

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
        fn _match_process_input(addr: &str)
        {
            match process_input(addr) {
                StringOrVec::String(addr) => resolve_results(&addr),
                StringOrVec::Vec(addrs) => for addr in addrs {resolve_results(&addr);},
            }
        }
        match v {
            json::JsonValue::Short(addr) => {
                _match_process_input(addr.as_str());
            },
            json::JsonValue::String(addr) => {
                _match_process_input(addr.as_str());
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