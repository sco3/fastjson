use dirs;
use serde_json::from_str;
use serde_json::Value;
use std::fs::read_to_string;
use std::fs::File;
use std::time::Instant;

fn _get_int(v: Value, s: &str) -> i64 {
    return v[s].as_i64().unwrap_or(0);
}

fn run() {
    let file_path = dirs::home_dir() //
        .unwrap()
        .join(".local/devices.json");

    let s = read_to_string(file_path) //
        .unwrap_or_default();

    let mut json: Value = from_str(&s).unwrap_or_default();
    //let _v = get_int(json, "returnCode");

    if let Some(rc) = json.get_mut("returnCode") {
        *rc = Value::from(1);
    }

    let filename = dirs::home_dir().unwrap().join(".local/devices-out.json");
    let mut file = File::create(filename).unwrap();

    serde_json::to_writer(&mut file, &json).unwrap();
}

fn main() {
    let timer = Instant::now();
    let n = 100;
    for _i in 0..n {
        let _ = run();
    }
    let took = timer.elapsed().as_millis();
    println!("Time: {} ms avg: {}", took, took / n);
}
