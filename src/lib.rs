use rand::Rng;
use std::usize;

#[inline]
pub fn monte_claro(size: usize) -> usize {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            let x: f64 = rng.gen_range(-1.0..=1.0);
            let y: f64 = rng.gen_range(-1.0..=1.0);

            if x * x + y * y <= 1.0 {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(not(feature = "single-threaded"))]
pub fn benchmark(size: usize, num_thread: usize) -> f64 {
    let mut workloads = vec![size / num_thread; num_thread];
    workloads[0] += size % num_thread;
    std::thread::scope(|s| {
        workloads
            .into_iter()
            .map(|workloads| s.spawn(move || monte_claro(workloads)))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum::<usize>() as f64
            / size as f64
            * 4.0
    })
}

#[cfg(feature = "single-threaded")]
pub fn benchmark(size: usize, _: usize) -> f64 {
    monte_claro(size) as f64 / size as f64 * 4.0
}

#[test]
fn test_monte_claro() {
    let size = 1000000;
    let pi = monte_claro(size) as f64 / size as f64 * 4.0;
    assert!((pi - std::f64::consts::PI).abs() < 0.01);
}

#[test]
fn test_monte_claro_benchmark() {
    let size = 1000000;
    let num_thread = 4;
    let pi = benchmark(size, num_thread);
    assert!((pi - std::f64::consts::PI).abs() < 0.01);
}
