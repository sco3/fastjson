use dirs;
use serde_json::from_str;
use serde_json::Value;
use std::fs::read_to_string;
use std::time::Instant;

fn get_int(v: Value, s: &str) -> i64 {
    return v[s].as_i64().unwrap_or(0);
}
fn run() {
    let file_path = dirs::home_dir() //
        .unwrap()
        .join(".local/devices.json");

    let s = read_to_string(file_path) //
        .unwrap_or_default();

    let json: Value = from_str(&s).unwrap();
    let _v = get_int(json, "returnCode");

    //println!("{}", v);
}

fn main() {
    let timer = Instant::now();
    let n = 1000;
    for _i in 0..n {
        let _ = run();
    }
    println!("Time: {} ms", timer.elapsed().as_millis());
}
