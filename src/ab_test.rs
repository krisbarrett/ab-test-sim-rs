use crate::stats::prop_test;
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;
use rand::rngs::SmallRng;
use rand::Rng;

pub struct ABTest {
    ca: u32,
    cb: u32,
    na: u32,
    nb: u32,
    n: u32,
    a_dist: Bernoulli,
    b_dist: Bernoulli,
}

impl ABTest {
    pub fn new(a: f64, b: f64, required_n: u32) -> ABTest {
        ABTest{ca: 0, cb: 0, na: 0, nb: 0, n: required_n, a_dist: Bernoulli::new(a).unwrap(), b_dist: Bernoulli::new(b).unwrap()}
    }

    pub fn next(&mut self, rng: &mut SmallRng) -> bool {
        if rng.gen::<f64>() < 0.5 {
            self.trial_a(rng);
        } else {
            self.trial_b(rng);
        }

        self.na < self.n || self.nb < self.n
    }

    fn trial_a(&mut self, rng: &mut SmallRng) {
        self.ca += self.a_dist.sample(rng) as u32;
        self.na += 1;
    }

    fn trial_b(&mut self, rng: &mut SmallRng) {
        self.cb += self.b_dist.sample(rng) as u32;
        self.nb += 1;
    }

    pub fn p_value(&self) -> f64 {
        prop_test(self.ca, self.na, self.cb, self.nb).unwrap()
    }
}