use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file-path>", args[0]);
        eprintln!("Example: {} /tmp/wasm-test.txt", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    println!("WASM module attempting to read: {}", file_path);
    println!("(This requires an explicit --dir capability grant from the host)\n");

    let content = fs::read_to_string(file_path)?;

    println!("✅ Successfully read {} bytes:", content.len());
    println!("─────────────────────────────");
    print!("{}", content);
    println!("─────────────────────────────");

    Ok(())
}
