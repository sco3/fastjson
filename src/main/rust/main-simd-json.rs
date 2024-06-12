use serde_json::Value;
use simd_json;
use std::fs;
use std::fs::File;
use std::time::Instant;

fn run() {
    let file_path = dirs::home_dir() //
        .unwrap()
        .join(".local/devices.json");

    let mut buf = fs::read(file_path).unwrap();
    //let v: simd_json::OwnedValue = simd_json::to_owned_value(&mut buf).unwrap();
    //let v: simd_json::BorrowedValue = simd_json::to_borrowed_value(&mut buf).unwrap();
    let v: Value = simd_json::serde::from_slice(&mut buf).unwrap();

    let filename = dirs::home_dir().unwrap().join(".local/devices-out.json");
    let mut outfile = File::create(filename).unwrap();

    simd_json::to_writer(&mut outfile, &v).unwrap();
}

fn main() {
    let timer = Instant::now();
    let n = 100;
    for _i in 0..n {
        let _ = run();
    }
    let took = timer.elapsed().as_millis();
    println!("Time: {} ms avg: {} runs: {}", took, took / n, n);
}
