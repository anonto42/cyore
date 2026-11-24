/// Sigmoid: squashes any number into (0, 1)
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Derivative of sigmoid, written in terms of its OUTPUT.
/// Pass in s = sigmoid(x), not x itself.
pub fn sigmoid_derivative(s: f64) -> f64 {
    s * (1.0 - s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid_zero() {
        let result = sigmoid(0.0);
        assert!((result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_sigmoid_bounds() {
        assert!(sigmoid(100.0) > 0.999);
        assert!(sigmoid(-100.0) < 0.001);
    }

    #[test]
    fn test_sigmoid_derivative() {
        // sigmoid(0.0) = 0.5; derivative at 0.5 is 0.5 * (1.0 - 0.5) = 0.25
        let s = sigmoid(0.0);
        let deriv = sigmoid_derivative(s);
        assert!((deriv - 0.25).abs() < 1e-6);
    }
}