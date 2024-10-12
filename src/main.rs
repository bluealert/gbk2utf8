use encoding_rs::{GBK, UTF_8};
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

    let (_, _, not_utf8) = UTF_8.decode(&content);
    if not_utf8 {
        let (result, _, not_gbk) = GBK.decode(&content);
        if not_gbk {
            println!("Failed to decode file as GBK: {}", file_path);
            return Ok(());
        }
        fs::write(file_path, result.into_owned())?;
    }
    Ok(())
}
