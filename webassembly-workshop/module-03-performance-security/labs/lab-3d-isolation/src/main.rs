use wasmtime::*;
use anyhow::Result;

fn main() -> Result<()> {
    let wasm_path = "wasm-guest/target/wasm32-wasip1/release/wasm_guest_isolation.wasm";
    let engine = Engine::default();
    let module = Module::from_file(&engine, wasm_path)?;

    println!("Loading the SAME .wasm module into TWO separate instances...");
    let mut store_a = Store::new(&engine, ());
    let mut store_b = Store::new(&engine, ());

    let instance_a = Instance::new(&mut store_a, &module, &[])?;
    let instance_b = Instance::new(&mut store_b, &module, &[])?;

    println!("✅ Instance A and Instance B created\n");

    // Get function handles
    let write_a = instance_a.get_typed_func::<(i32, i32), ()>(&mut store_a, "write_i32")?;
    let read_a  = instance_a.get_typed_func::<i32, i32>(&mut store_a, "read_i32")?;
    let read_b  = instance_b.get_typed_func::<i32, i32>(&mut store_b, "read_i32")?;

    let secret: i32 = 0x53_45_43_52; // "SECR" in ASCII as i32
    let addr:   i32 = 0;              // write at address 0

    // Write secret into Instance A's memory
    println!("Writing secret value ({:#010X}) into Instance A at address {}...", secret, addr);
    write_a.call(&mut store_a, (addr, secret))?;

    // Read back from Instance A (should match)
    let val_a = read_a.call(&mut store_a, addr)?;
    println!("Instance A reads address {}: {:#010X} ✅ (matches secret)", addr, val_a);

    // Read from Instance B's memory (should be 0 — completely isolated)
    let val_b = read_b.call(&mut store_b, addr)?;
    println!("Instance B reads address {}: {:#010X}", addr, val_b);

    if val_b == 0 {
        println!("\n🔒 ISOLATION CONFIRMED: Instance B cannot see Instance A's memory!");
        println!("   Each WASM instance has its own private linear memory.");
    } else {
        println!("\n⚠️  WARNING: Unexpected value in Instance B — isolation may be broken!");
    }

    Ok(())
}
