pub fn calculate_interference(a: f64, b: f64) -> f64 {
    (a + b).sin() * (a - b).cos()
}

pub fn calculate_amplitude(a: f64, b: f64, phase_diff: f64) -> f64 {
    (a * a + b * b + 2.0 * a * b * phase_diff.cos()).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference() {
        let result = calculate_interference(1.0, 2.0);
        assert!(result.is_finite());
    }
}
