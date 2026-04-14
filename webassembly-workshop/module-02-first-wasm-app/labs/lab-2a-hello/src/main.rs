fn main() {
    println!("Hello from WebAssembly! 🚀");
    println!("Running inside a WASM sandbox.");

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("Arguments received:");
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("  [{}] {}", i, arg);
        }
    }
}
