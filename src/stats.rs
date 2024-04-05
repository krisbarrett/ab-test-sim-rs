use statrs::distribution::{ContinuousCDF, Gamma};

/// prop_test is the test of equal or given proportions. It returns a p-value that represents the
/// probability that the observed result was due to chance. Note that a p-value < 0.05 is generally
/// considered to be statistically significant.
pub fn prop_test(control_conversions: u32, control_views: u32, variation_conversions: u32, variation_views: u32) -> Result<f64, &'static str> {
    if control_conversions == 0 || variation_conversions == 0 {
        return Err("conversions must be greater than 0")
    }
    if control_views < control_conversions || variation_views < variation_conversions {
        return Err("views must be greater than or equal to conversions")
    }

    let a = (control_views - control_conversions) as f64;
    let b = (variation_views - variation_conversions) as f64;
    let c = (control_conversions) as f64;
    let d = (variation_conversions) as f64;
    let n = a + b + c + d;

    let xa = (a + b) * (a + c) / n;
    let xb = (a + b) * (b + d) / n;
    let xc = (a + c) * (c + d) / n;
    let xd = (b + d) * (c + d) / n;

    let q1 =  (a - xa).powf(2.0) / xa;
    let q2 = (b - xb).powf(2.0) / xb;
    let q3 = (c - xc).powf(2.0) / xc;
    let q4 = (d - xd).powf(2.0) / xd;

    let chi_sqr = q1 + q2 + q3 + q4;
    let gamma = Gamma::new(0.5, 0.5).unwrap();
    let p = 1.0 - gamma.cdf(chi_sqr);

    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_errors_with_invalid_conversions() {
        let result = prop_test(0, 100, 0, 100);
        match result {
            Ok(_) => panic!("expected an error, but got Ok"),
            Err(e) => assert_eq!(e, "conversions must be greater than 0")
        }

        let result = prop_test(101, 100, 101, 100);
        match result {
            Ok(_) => panic!("expected an error, but got Ok"),
            Err(e) => assert_eq!(e, "views must be greater than or equal to conversions")
        }

        // TODO: move this to valid test case
        let result = prop_test(100, 100, 100, 100);
        match result {
            Ok(p) => assert!(p.is_nan()),
            Err(e) => panic!("expected Ok, but got '{}'", e)
        }
    }
}
