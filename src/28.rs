use std::fs;
use std::io;

fn main() -> std::io::Result<()> {
    let filename = "example.txt";
    let content = fs::read_to_string(filename)?;
    
    println!("Read file: {}", filename);
    println!("{}", content);
    
    Ok(())
}
