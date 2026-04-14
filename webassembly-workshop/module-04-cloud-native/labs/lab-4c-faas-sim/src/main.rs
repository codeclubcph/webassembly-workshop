use wasmtime::*;
use std::io::{self, BufRead};
use std::time::Instant;

const WASM_PATH: &str = "event-handler/target/wasm32-wasip1/release/event_handler.wasm";

fn instantiate(engine: &Engine, module: &Module) -> Result<(Store<()>, Instance), anyhow::Error> {
    let mut store = Store::new(engine, ());
    let instance = Instance::new(&mut store, module, &[])?;
    Ok((store, instance))
}

fn invoke(store: &mut Store<()>, instance: &Instance, event_json: &str) -> Result<String, anyhow::Error> {
    let alloc = instance.get_typed_func::<i32, i32>(store, "alloc")?;
    let handle_event = instance.get_typed_func::<(i32, i32), i64>(store, "handle_event")?;

    let bytes = event_json.as_bytes();
    let ptr = alloc.call(store, bytes.len() as i32)?;

    let memory = instance.get_memory(store, "memory")
        .ok_or_else(|| anyhow::anyhow!("no memory"))?;
    memory.write(store, ptr as usize, bytes)?;

    let packed = handle_event.call(store, (ptr, bytes.len() as i32))?;
    let out_ptr = (packed >> 32) as usize;
    let out_len = (packed & 0xFFFF_FFFF) as usize;

    let mut buf = vec![0u8; out_len];
    memory.read(store, out_ptr, &mut buf)?;
    Ok(String::from_utf8(buf)?)
}

fn main() -> anyhow::Result<()> {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║   WASM FaaS Simulator – Cold vs Warm Invocation ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();
    println!("Type a JSON event payload and press Enter to invoke.");
    println!("Type 'quit' to exit.\n");
    println!("Example: {{\"event\": \"order.placed\", \"orderId\": \"abc123\"}}");
    println!("─────────────────────────────────────────────────────\n");

    let engine = Engine::default();

    // Load module once (simulates the module being cached)
    let module_load_start = Instant::now();
    let module = Module::from_file(&engine, WASM_PATH)?;
    let module_load_time = module_load_start.elapsed();
    println!("📦 Module loaded in {:.2}ms (one-time cost)\n", module_load_time.as_secs_f64() * 1000.0);

    // Pre-warm one instance
    let warm_start = Instant::now();
    let (mut warm_store, warm_instance) = instantiate(&engine, &module)?;
    let warm_time = warm_start.elapsed();
    println!("🔥 Warm instance ready in {:.2}ms\n", warm_time.as_secs_f64() * 1000.0);

    let stdin = io::stdin();
    let mut invocation = 0u32;

    for line in stdin.lock().lines() {
        let input = line?.trim().to_string();
        if input == "quit" || input == "exit" { break; }
        if input.is_empty() { continue; }

        invocation += 1;
        println!("\n[Invocation #{}]", invocation);

        // ── COLD path: instantiate fresh module ──────────────────────────
        let cold_start = Instant::now();
        let (mut cold_store, cold_instance) = instantiate(&engine, &module)?;
        let cold_instance_time = cold_start.elapsed();

        let exec_start = Instant::now();
        match invoke(&mut cold_store, &cold_instance, &input) {
            Ok(result) => {
                let exec_time = exec_start.elapsed();
                println!("  🥶 COLD  – instantiate: {:.2}ms | exec: {:.3}ms | result: {}",
                    cold_instance_time.as_secs_f64() * 1000.0,
                    exec_time.as_secs_f64() * 1000.0,
                    result);
            }
            Err(e) => println!("  🥶 COLD  – error: {}", e),
        }

        // ── WARM path: reuse pre-instantiated module ──────────────────────
        let warm_exec_start = Instant::now();
        match invoke(&mut warm_store, &warm_instance, &input) {
            Ok(result) => {
                let warm_exec_time = warm_exec_start.elapsed();
                println!("  🔥 WARM  – instantiate: 0.00ms | exec: {:.3}ms | result: {}",
                    warm_exec_time.as_secs_f64() * 1000.0,
                    result);
            }
            Err(e) => println!("  🔥 WARM  – error: {}", e),
        }
    }

    println!("\nSimulator stopped after {} invocation(s).", invocation);
    Ok(())
}
