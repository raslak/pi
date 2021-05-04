// Go code from https://ggcarvalho.dev/posts/montecarlo/ rewritten in Rust.

use rand::Rng;
use std::f64::consts::PI;

fn main() {
    let points: i32 = 1_000_000;
    println!("Estimating pi with {} point(s).", points);

    let mut success: i32 = 0;

    for _ in 0..points {
        let (x, y): (f64, f64) = gen_random_point();

        // Check if point lies within the circular region:
        if x * x + y * y < 1.0 {
            success += 1;
        }
    }

    let pi_approx: f64 = 4.0 * (success as f64 / points as f64);
    let error_pct: f64 = 100.0 * (pi_approx - PI).abs() / PI;

    println!("Estimated pi: {:.5}", pi_approx);
    println!("pi: {:.5}", PI);
    println!("Error: {:.5}", error_pct);
}

fn gen_random_point() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    (2.0 * x - 1.0, 2.0 * y - 1.0)
}
