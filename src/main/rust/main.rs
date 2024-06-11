use dirs;


use std::fs::File;
use std::io;
use std::io::Read;

use std::time::Instant;

fn run() -> io::Result<()> {
    let timer = Instant::now();
    let home_dir = dirs::home_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Home directory not found",
    ))?;
    let file_path = home_dir.join("/.local/devices.json");
    let mut file = File::open(&file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("Read: {}", String::from_utf8_lossy(&buffer));

    println!("Time2: {} ms", timer.elapsed().as_millis());
    Ok(())
}

fn main() {
    run();
}
