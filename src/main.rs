#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    Api::write_string("### Hello, HTTP Client\n");
    println!("### Hello, HTTP Client from println!");
    
    let client = HttpClient::new();
    
    match client.get("host.test".to_string(), 8000, "/test.html".to_string()) {
    //match client.get("example.com".to_string(), 80, "/".to_string()) {
        Ok(res) => {
            print!("response:\n{:#?}", res);
        }
        Err(e) => {
            print!("error:\n{:#?}", e);
        }
    }
    Api::write_string("### Bye HTTP Client \n");

    // exit
    0
}

entry_point!(main);