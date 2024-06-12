use dirs;
use serde_json::Value;
use std::fs::read_to_string;
use std::io;
use std::time::Instant;

fn run() -> io::Result<()> {
    let home_dir = dirs::home_dir();
    let file_path = home_dir.unwrap().join(".local/devices.json");
    let s = read_to_string(file_path)?;

    let json: Value = serde_json::from_str(&s)?;
    let tag_name = "returnCode"; // Replace with the actual tag name
    match json.get(tag_name) {
        Some(value) => println!("Value for '{}': {}", tag_name, value),
        None => println!("Tag '{}' not found in the JSON file", tag_name),
    }
    Ok(())
}

fn main() {
    let timer = Instant::now();
    let n = 1;
    for _i in 0..n {
        let _ = run();
    }
    println!("Time: {} ms", timer.elapsed().as_millis());
}
