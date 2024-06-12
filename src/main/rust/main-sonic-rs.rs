use sonic_rs::{from_str, JsonValueTrait, Value};
use std::fs::read_to_string;

fn main() {
    let json = r#"{
        "returnCode": 18
    }"#;

    let file_path = dirs::home_dir() //
        .unwrap()
        .join(".local/devices.json");

    let s = read_to_string(file_path) //
        .unwrap_or_default();

    let root: Value = from_str(json).unwrap();

    let v = root.get("returnCode").as_i64().unwrap_or_default();
    println!("v: {}", v);
}
