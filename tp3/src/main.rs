use std::fs::File;
use std::io::{self, Write};


fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;

    file.write_all(b"Hello, world!")?;
    file.write_all(b"\nThis is a test file.")?;

    println!("File written successfully!");

    Ok(())
}
