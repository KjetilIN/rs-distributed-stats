use std::{env, fs::File, io::Read};
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }
    let file_path = &args[1];

    // Open the file asynchronously
    let mut file: File = File::open(file_path)?;
    
    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Process the file contents
    println!("File Contents: {}", contents);
    
    // Further processing can be done here

    Ok(())
}
