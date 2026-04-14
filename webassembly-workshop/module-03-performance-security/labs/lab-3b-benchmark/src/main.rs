use std::time::Instant;

/// Recursive fibonacci – intentionally CPU-bound for benchmarking
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let n = 40u64;

    println!("Computing fibonacci({})...", n);

    let start = Instant::now();
    let result = fibonacci(n);
    let elapsed = start.elapsed();

    println!("fibonacci({}) = {}", n, result);
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
