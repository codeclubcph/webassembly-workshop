/// Simulates a "malicious" or untrusted WASM module attempting to
/// access resources beyond its granted capabilities.
///
/// In a real attack scenario, this module would be supplied by an
/// untrusted third party. The WASM sandbox ensures it cannot cause harm.

use std::env;
use std::fs;

fn main() {
    println!("=== Untrusted WASM Module Security Demo ===\n");

    // Attempt 1: Read a sensitive file
    println!("Attempt 1: Reading /etc/passwd ...");
    match fs::read_to_string("/etc/passwd") {
        Ok(content) => println!("  ⚠️  SUCCESS (unexpected!): {} bytes read", content.len()),
        Err(e)      => println!("  ✅ BLOCKED: {}", e),
    }

    // Attempt 2: Read a file outside granted dirs
    println!("\nAttempt 2: Reading /etc/shadow ...");
    match fs::read_to_string("/etc/shadow") {
        Ok(_)  => println!("  ⚠️  SUCCESS (unexpected!)"),
        Err(e) => println!("  ✅ BLOCKED: {}", e),
    }

    // Attempt 3: Write to /tmp (blocked unless --dir /tmp granted)
    println!("\nAttempt 3: Writing to /tmp/pwned.txt ...");
    match fs::write("/tmp/pwned.txt", "you have been pwned") {
        Ok(_)  => println!("  ⚠️  SUCCESS: file written (only if --dir /tmp was granted)"),
        Err(e) => println!("  ✅ BLOCKED: {}", e),
    }

    // Attempt 4: Read env vars
    println!("\nAttempt 4: Reading environment variables ...");
    let vars: Vec<(String, String)> = env::vars().collect();
    if vars.is_empty() {
        println!("  ✅ No env vars visible (none granted by host)");
    } else {
        println!("  Visible env vars ({} total):", vars.len());
        for (k, v) in &vars {
            println!("    {} = {}", k, v);
        }
    }

    // Attempt 5: Read a specific env var that might hold secrets
    println!("\nAttempt 5: Reading SECRET_TOKEN env var ...");
    match env::var("SECRET_TOKEN") {
        Ok(v)  => println!("  ⚠️  Got SECRET_TOKEN: {}", v),
        Err(_) => println!("  ✅ SECRET_TOKEN not visible in sandbox"),
    }

    println!("\n=== Demo complete ===");
    println!("WASM sandbox has limited what this module can do.");
}
