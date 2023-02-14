use std::sync::{Mutex, Arc, Barrier};

use colored::*;
use json::JsonValue;
use threadpool::ThreadPool;

use super::tcp::*;

pub fn read_from_json_sync(json: &json::JsonValue) {
    match json {
        json::JsonValue::Object(root) => read_object_sync(&root),
        json::JsonValue::Array(root) => read_adresses_sync(&root),
        _ => panic!("Root is not an object or array"),
    };
}

pub unsafe fn read_from_json(json: &json::JsonValue, workers: usize) {
    let mut stack: Vec<(String, Vec<JsonValue>)> = Vec::new();
    
    match json {
        json::JsonValue::Object(root) => stack = [stack, read_object(root)].concat(),
        json::JsonValue::Array(root) => stack.push((String::from("Adresses"), root.clone())),
        _ => panic!("Root is not an object or array"),
    };
    let pool = ThreadPool::new(workers);

    let lc = Arc::new(Mutex::new(()));
    let barrier = Arc::new(Barrier::new(stack.len()+1));

    for (key, values) in stack {
        let lc = lc.clone();
        let barrier = barrier.clone();
        let _v = values.clone();

        pool.execute(move|| {
            let mut message = format!("[{}]:\n", key.bold().yellow());
            // let mut message = String::from("");
            read_adresses(&mut message, &_v);
            match lc.lock() {
                Ok(_) => println!("{}\n", message),
                Err(_) => panic!("Unable to lock mutex"),
            }
            barrier.wait();
        });
    }
    barrier.wait();
}

pub fn read_adress(message: &mut String, addr: &str) {
    match process_input(addr) {
        StringOrVec::String(addr) => resolve_results_async(message, &addr),
        StringOrVec::Vec(addrs) => for addr in addrs {resolve_results_async(message, &addr);},
    }
}

pub fn read_adress_sync(addr: &str) {
    match process_input(addr) {
        StringOrVec::String(addr) => resolve_results(&addr),
        StringOrVec::Vec(addrs) => for addr in addrs {resolve_results(&addr);},
    }
}

pub fn read_adresses(message: &mut String, values: &Vec<json::JsonValue>) {
    for v in values.clone() {
        match v {
            json::JsonValue::Short(addr) => {
                read_adress(message, addr.as_str());
            },
            json::JsonValue::String(addr) => {
                read_adress(message, addr.as_str());
            },
            _ => ()
        }
    }
}

pub fn read_adresses_sync(values: &Vec<json::JsonValue>) {
    for v in values.clone() {
        match v {
            json::JsonValue::Short(addr) => {
                read_adress_sync(addr.as_str());
            },
            json::JsonValue::String(addr) => {
                read_adress_sync(addr.as_str());
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
            json::JsonValue::Array(v) => {read_adresses_sync(v); println!("");},
            json::JsonValue::Short(addr) => {read_adress_sync(addr.as_str());},
            json::JsonValue::String(addr) => {read_adress_sync(addr.as_str());},
            _ => ()
        }
    }
}

pub unsafe fn read_object(obj: &json::object::Object)-> Vec<(String, Vec<JsonValue>)> {
    let mut stack: Vec<(String, Vec<JsonValue>)> = Vec::new();
    for (key, value) in obj.iter() {
        match value {
            json::JsonValue::Object(v) => stack = [stack, read_object(v)].concat(),
            json::JsonValue::Array(v) => stack.push((String::from(key), v.clone())),
            JsonValue::Short(adrr) => stack.push((String::from(key), vec![JsonValue::Short(adrr.clone())])),
            JsonValue::String(adrr) => stack.push((String::from(key), vec![JsonValue::String(adrr.clone())])),
            _ => ()
        }
    }
    stack
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