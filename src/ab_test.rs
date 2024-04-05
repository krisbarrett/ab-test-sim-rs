use crate::stats::prop_test;
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;
use rand::rngs::SmallRng;
use rand::Rng;

/// ABTest represents an A/B test
pub struct ABTest {
    ca: u32, // conversions for a
    cb: u32, // conversions for b
    na: u32, // views for a
    nb: u32, // views for b
    n: u32, // required sample size
    a_dist: Bernoulli, // Bernoulli distribution for a
    b_dist: Bernoulli, // Bernoulli distribution for b
}

impl ABTest {
    /// new creates a new ABTest with simulated conversion rates "a" and "b" with required sample
    /// size "n"
    pub fn new(a: f64, b: f64, n: u32) -> ABTest {
        ABTest{ca: 0, cb: 0, na: 0, nb: 0, n: n, a_dist: Bernoulli::new(a).unwrap(), b_dist: Bernoulli::new(b).unwrap()}
    }

    /// next randomly selects between A and B with equal probability and simulates a trial. It
    /// returns true if the A/B test should continue running
    pub fn next(&mut self, rng: &mut SmallRng) -> bool {
        if rng.gen::<f64>() < 0.5 {
            self.trial_a(rng);
        } else {
            self.trial_b(rng);
        }

        self.na < self.n || self.nb < self.n
    }

    /// trial_a runs a trial for A
    fn trial_a(&mut self, rng: &mut SmallRng) {
        self.ca += self.a_dist.sample(rng) as u32;
        self.na += 1;
    }

    /// trial_b runs a trial for B
    fn trial_b(&mut self, rng: &mut SmallRng) {
        self.cb += self.b_dist.sample(rng) as u32;
        self.nb += 1;
    }

    /// p_value runs a test of equal or given proportions and returns the resulting p-value
    pub fn p_value(&self) -> f64 {
        prop_test(self.ca, self.na, self.cb, self.nb).unwrap()
    }
}