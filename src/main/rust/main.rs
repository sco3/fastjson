use dirs;
use serde_json::from_str;
use serde_json::Value;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
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

    let mut file = File::create(
        dirs::home_dir() //
            .unwrap()
            .join(".local/devices-out.json"),
    );
    match file {
        Ok(file) => file.write_all(json.as_bytes()),
        Err(e) => Ok(),
    }

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
