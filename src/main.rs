use std::thread;
use rand::SeedableRng;
use rand::rngs::SmallRng;

mod stats;
mod ab_test;

fn run_simulation(sims: u32, a: f64, b: f64, required_n: u32) -> u32 {
    let mut diffs = 0;
    let mut rng = SmallRng::from_entropy();

    for _ in 0..sims {
        let mut ab = ab_test::ABTest::new(a, b, required_n);

        while ab.next(&mut rng) {}

        let p_value = ab.p_value();
        if !p_value.is_nan() && p_value < 0.05 {
            diffs += 1;
        }
    }

    diffs
}

fn simulate(label: &str, total_sims: u32, a: f64, b: f64, required_n: u32) {
    let mut diffs = 0;
    let num_threads = thread::available_parallelism().unwrap().get() as u32;
    let num_sims = total_sims / num_threads;

    let mut handles: Vec<thread::JoinHandle<u32>> = Vec::new();
    for _ in 0..num_threads {
        let handle = thread::spawn(move || run_simulation(num_sims, a, b, required_n));
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
    let required_n = 25580;


    simulate("false positive rate", total_sims, a, a, required_n);
    simulate("true positive rate", total_sims, a, b, required_n);
}
