use std::env;

fn main() {
    println!("=== WASM Environment & Args Demo ===\n");

    // Print environment variables
    println!("Environment variables:");
    for (key, value) in env::vars() {
        println!("  {} = {}", key, value);
    }

    // Print command-line arguments
    println!("\nCommand-line arguments:");
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("  [{}] {}", i, arg);
    }

    // Read a specific env var
    match env::var("APP_NAME") {
        Ok(val) => println!("\nAPP_NAME is set to: {}", val),
        Err(_)  => println!("\nAPP_NAME is not set"),
    }

    match env::var("LOG_LEVEL") {
        Ok(val) => println!("LOG_LEVEL is set to: {}", val),
        Err(_)  => println!("LOG_LEVEL is not set"),
    }
}
