use std::fs::File;
use std::io::{self, Write};

fn write_file() -> io::Result<()> {
    let file = File::create("foo.txt")?;
    take_and_write_file(file)?;
    Ok(())
}

fn take_and_write_file(mut file: File) -> io::Result<()>{
    file.write_all(b"Hello, world!")?;
    Ok(())
}