extern crate reqwest;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;

// Entry point
fn main() {
    let mut json_text = String::new();
    // Insert a character string in "UserData.json" into json_text.
    match read_file("UserData.json")
        .read_to_string(&mut json_text)
        {
            Ok(_) => println!("JSON read complete."),
            Err(why) => panic!("{}", Error::description(&why)),
        }
    let mut response_data = send_post_request(
        "http://10.0.0.1/login",
        serde_json::from_str(&json_text).unwrap(),
    );
    println!("{}", response_data.text().unwrap());
}

fn read_file(path_str: &str) -> File {
    let path_obj = Path::new(path_str);
    match File::open(path_obj) {
        Ok(file) => file,
        Err(why) => panic!("couldn't read {}: {}",
                           path_obj.display(),
                           Error::description(&why)
        ),
    }
}

fn send_post_request(url: &str,
                     send_data: BTreeMap<String, String>) -> reqwest::Response {
    reqwest::Client::new().post(url)
        .form(&send_data)
        .send()
        .unwrap()
}
