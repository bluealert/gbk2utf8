use encoding_rs::GBK;
use std::{
    env, fs,
    io::{self},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];
    let content = fs::read(file_path)?;

    match std::str::from_utf8(&content) {
        Ok(_) => {}
        Err(_) => {
            let (result, _, failed) = GBK.decode(&content);
            if failed {
                println!("Failed to decode file as GBK: {}", file_path);
                return Ok(());
            }

            fs::write(file_path, result.into_owned())?;
        }
    }

    Ok(())
}
