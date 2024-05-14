use monte_carlo_benchmark::benchmark;
fn main() {
    // do something like this
    let size = 1000000;
    let num_thread = 8;
    let pi = benchmark(size, num_thread);
    println!("pi = {}", pi);
}
