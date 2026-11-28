/// Sigmoid: squashes any number into (0, 1).
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Derivative of sigmoid, written in terms of its OUTPUT.
/// Pass in s = sigmoid(x), not x itself: σ′(x) = σ(x) · (1 − σ(x)).
pub fn sigmoid_derivative(s: f64) -> f64 {
    s * (1.0 - s)
}

/// Softmax: turns raw scores into probabilities that are all ≥ 0 and sum to 1.
///
/// p_k = e^(z_k − m) / Σ_j e^(z_j − m), where m = max_j z_j.
/// Subtracting m does not change the result (it cancels in the fraction), but it keeps every
/// exponent ≤ 0, so e^(…) never overflows to infinity for large scores.
pub fn softmax(scores: &[f64], out: &mut [f64]) {
    assert_eq!(scores.len(), out.len(), "softmax: length mismatch");
    let max = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut sum = 0.0;
    for (o, &z) in out.iter_mut().zip(scores) {
        *o = (z - max).exp();
        sum += *o;
    }
    for o in out.iter_mut() {
        *o /= sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid_zero() {
        assert!((sigmoid(0.0) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_sigmoid_derivative() {
        assert!((sigmoid_derivative(sigmoid(0.0)) - 0.25).abs() < 1e-12);
    }

    #[test]
    fn test_softmax_is_distribution() {
        let scores = [1.0, 2.0, 3.0, -1.0, 0.5];
        let mut p = [0.0; 5];
        softmax(&scores, &mut p);
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert!(p.iter().all(|&x| x >= 0.0));
        // Bigger score, bigger probability.
        assert!(p[2] > p[1] && p[1] > p[0]);
    }

    #[test]
    fn test_softmax_equal_scores_uniform() {
        let mut p = [0.0; 4];
        softmax(&[7.0; 4], &mut p);
        for x in p {
            assert!((x - 0.25).abs() < 1e-12);
        }
    }

    #[test]
    fn test_softmax_large_scores_finite() {
        // Without subtracting the max, e^1000 would be infinity and the result NaN.
        let mut p = [0.0; 3];
        softmax(&[1000.0, 999.0, -1000.0], &mut p);
        assert!(p.iter().all(|x| x.is_finite()));
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        // e^0 / (e^0 + e^-1) ≈ 0.7311
        assert!((p[0] - 1.0 / (1.0 + (-1.0f64).exp())).abs() < 1e-9);
    }
}
