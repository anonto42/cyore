/// Smallest probability we take the log of; ln(0) would be −∞.
const MIN_PROBABILITY: f64 = 1e-15;

/// Cross-entropy loss for one example: L = −Σ_k y_k ln p_k.
///
/// The target y is one-hot (1 for the true label, 0 elsewhere), so the sum collapses to
/// L = −ln p_label: the loss is small when the network gives the right digit a high probability.
pub fn cross_entropy(probabilities: &[f64], label: usize) -> f64 {
    -probabilities[label].max(MIN_PROBABILITY).ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_prediction() {
        let mut p = [0.0; 10];
        p[3] = 1.0;
        assert!(cross_entropy(&p, 3).abs() < 1e-12);
    }

    #[test]
    fn test_uniform_prediction() {
        // A network that knows nothing gives every digit 1/10: loss = −ln(0.1) = ln 10.
        let p = [0.1; 10];
        assert!((cross_entropy(&p, 7) - 10f64.ln()).abs() < 1e-12);
    }

    #[test]
    fn test_zero_probability_is_finite() {
        let mut p = [0.0; 10];
        p[0] = 1.0;
        let loss = cross_entropy(&p, 5);
        assert!(loss.is_finite());
        assert!((loss - (-MIN_PROBABILITY.ln())).abs() < 1e-9);
    }
}
