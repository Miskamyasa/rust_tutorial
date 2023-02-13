#![allow(unused)]

use serde_json::{Number, Value};
use std::fmt::Display;
use std::fs;
use pad::PadStr;

const ERR_STR: &str = "Something went wrong reading / writing files";

fn open_file(file_name: &str) -> Value {
    let content = fs::read_to_string(file_name).unwrap();
    serde_json::from_str::<Value>(&content).unwrap()
}

fn write_file(file_name: &str, content: &Value) {
  let content = serde_json::to_string_pretty(&content).unwrap();
  fs::write(file_name, content).expect(ERR_STR);
}

fn main() {
    let mut app_json = open_file("app.json");

    let mut package_json = open_file("package.json");

    let ver = app_json["expo"]["version"]
        .as_str()
        .unwrap()
        .split('.')
        .collect::<Vec<&str>>();

    let old_ver = ver[2].parse::<i32>().unwrap();

    let new_ver = Value::String(format!("{}.{}.{}", ver[0], ver[1], old_ver + 1));

    let build_number = Number::from(
        app_json["expo"]["android"]["versionCode"].as_u64().unwrap() + 1,
    );

    app_json["expo"]["version"] = new_ver.clone();
    package_json["version"] = new_ver.clone();

    app_json["expo"]["ios"]["buildNumber"] = Value::String(build_number.clone().to_string());
    app_json["expo"]["android"]["versionCode"] = Value::Number(build_number.clone());

    write_file("app.json", &app_json);
    write_file("package.json", &package_json);

    println!("┌────────────────────────┐");
    println!("│ Build Number: {}│", build_number.to_string().pad_to_width(9));
    println!("│ Version: {}│", new_ver.to_string().pad_to_width(14));
    println!("└────────────────────────┘");
    
}
