use std::thread;
use rand::SeedableRng;
use rand::rngs::SmallRng;

mod stats;
mod ab_test;

/// run_simulation runs "num_sims" A/B tests with simulated conversion rates "a" and "b" with sample
/// size "n". It returns the number of statistically significant results that were observed.
fn run_simulation(sims: u32, a: f64, b: f64, n: u32) -> u32 {
    let mut diffs = 0;

    // SmallRng is used for its speed and our non-cryptographic use case
    let mut rng = SmallRng::from_entropy();

    for _ in 0..sims {
        let mut ab = ab_test::ABTest::new(a, b, n);

        while ab.next(&mut rng) {}

        let p_value = ab.p_value();
        if !p_value.is_nan() && p_value < 0.05 {
            diffs += 1;
        }
    }

    diffs
}

/// simulate runs "total_sims" A/B tests with simulated conversion rates "a" and "b" with sample
/// size "n". It prints the percentage of statistically significant results observed.
fn simulate(label: &str, total_sims: u32, a: f64, b: f64, n: u32) {
    let mut diffs = 0;
    let num_threads = thread::available_parallelism().unwrap().get() as u32;
    let num_sims = total_sims / num_threads;

    let mut handles: Vec<thread::JoinHandle<u32>> = Vec::new();
    for _ in 0..num_threads {
        let handle = thread::spawn(move || run_simulation(num_sims, a, b, n));
        handles.push(handle);
    }

    for handle in handles {
        diffs += handle.join().unwrap()
    }

    println!("{}: {}%", label, 100.0 * diffs as f64 / total_sims as f64)
}

fn main() {
    let total_sims: u32 = 10000;
    let mde  = 0.05;
    let a = 0.2;
    let b = a * (1.0+mde);
    let n = 25580;


    simulate("false positive rate", total_sims, a, a, n);
    simulate("true positive rate", total_sims, a, b, n);
}
