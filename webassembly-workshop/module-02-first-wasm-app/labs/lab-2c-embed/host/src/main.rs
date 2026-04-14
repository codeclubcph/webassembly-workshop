use wasmtime::*;
use anyhow::Result;

fn main() -> Result<()> {
    // Path to the compiled WASM guest
    let wasm_path = "wasm-guest/target/wasm32-wasip1/release/wasm_guest.wasm";

    println!("🔧 Initializing Wasmtime engine...");
    let engine = Engine::default();

    println!("📦 Loading WASM module: {}", wasm_path);
    let module = Module::from_file(&engine, wasm_path)?;

    println!("🏗  Instantiating module...");
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    // ── Call: add ──────────────────────────────────────────────────────────
    let add = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "add")?;

    println!("\n✅ Calling WASM function: add(10, 32)");
    let result = add.call(&mut store, (10, 32))?;
    println!("   Result: {}", result);  // expected: 42

    println!("✅ Calling WASM function: add(100, 200)");
    let result = add.call(&mut store, (100, 200))?;
    println!("   Result: {}", result);  // expected: 300

    // ── Call: multiply ─────────────────────────────────────────────────────
    let multiply = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "multiply")?;

    println!("\n✅ Calling WASM function: multiply(6, 7)");
    let result = multiply.call(&mut store, (6, 7))?;
    println!("   Result: {}", result);  // expected: 42

    // ── Call: fibonacci ────────────────────────────────────────────────────
    let fibonacci = instance
        .get_typed_func::<i32, i32>(&mut store, "fibonacci")?;

    println!("\n✅ Calling WASM function: fibonacci(10)");
    let result = fibonacci.call(&mut store, 10)?;
    println!("   Result: {}", result);  // expected: 55

    println!("\n🎉 All WASM calls completed successfully!");
    Ok(())
}
