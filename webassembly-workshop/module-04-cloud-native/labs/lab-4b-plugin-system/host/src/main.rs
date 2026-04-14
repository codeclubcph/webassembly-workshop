use wasmtime::*;
use anyhow::Result;
use std::env;

struct Plugin {
    name: String,
    instance: Instance,
    store: Store<()>,
}

impl Plugin {
    fn load(engine: &Engine, name: &str, path: &str) -> Result<Self> {
        let module = Module::from_file(engine, path)?;
        let mut store = Store::new(engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;
        Ok(Plugin { name: name.to_string(), instance, store })
    }

    fn transform(&mut self, input: &str) -> Result<String> {
        // Allocate memory in the WASM module for our input
        let alloc = self.instance
            .get_typed_func::<i32, i32>(&mut self.store, "alloc")?;
        let transform = self.instance
            .get_typed_func::<(i32, i32), i64>(&mut self.store, "transform")?;

        let input_bytes = input.as_bytes();
        let ptr = alloc.call(&mut self.store, input_bytes.len() as i32)?;

        // Write input bytes into WASM linear memory
        let memory = self.instance
            .get_memory(&mut self.store, "memory")
            .ok_or_else(|| anyhow::anyhow!("no memory export"))?;
        memory.write(&mut self.store, ptr as usize, input_bytes)?;

        // Call transform
        let packed = transform.call(&mut self.store, (ptr, input_bytes.len() as i32))?;
        let out_ptr = (packed >> 32) as usize;
        let out_len = (packed & 0xFFFF_FFFF) as usize;

        // Read result back
        let mut out_buf = vec![0u8; out_len];
        memory.read(&self.store, out_ptr, &mut out_buf)?;

        Ok(String::from_utf8(out_buf)?)
    }
}

fn main() -> Result<()> {
    let input = env::args().nth(1)
        .unwrap_or_else(|| "Hello from WebAssembly Cloud!".to_string());

    println!("🔌 WASM Plugin Pipeline Demo");
    println!("Input: {:?}\n", input);

    let engine = Engine::default();

    let plugin_specs = [
        ("uppercase", "plugins/uppercase/target/wasm32-wasip1/release/plugin_uppercase.wasm"),
        ("reverse",   "plugins/reverse/target/wasm32-wasip1/release/plugin_reverse.wasm"),
        ("wordcount", "plugins/wordcount/target/wasm32-wasip1/release/plugin_wordcount.wasm"),
    ];

    let mut current = input.clone();

    for (name, path) in &plugin_specs {
        let mut plugin = Plugin::load(&engine, name, path)?;
        let result = plugin.transform(&current)?;
        println!("Applying plugin: {}", name);
        println!("  → {:?}\n", result);
        current = result;
    }

    Ok(())
}
