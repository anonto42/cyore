/// Mean squared error: average of (prediction - target)^2
pub fn mse(predictions: &[f64], targets: &[f64]) -> f64 {
    let n = predictions.len() as f64;
    predictions
        .iter()
        .zip(targets)
        .map(|(p, t)| (p - t).powi(2))
        .sum::<f64>()
        / n
}

/// Derivative of MSE with respect to each prediction: 2 * (p - t) / n
pub fn mse_derivative(predictions: &[f64], targets: &[f64]) -> Vec<f64> {
    let n = predictions.len() as f64;
    predictions
        .iter()
        .zip(targets)
        .map(|(p, t)| 2.0 * (p - t) / n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse_exact_match() {
        let p = [1.0, 0.0];
        let t = [1.0, 0.0];
        assert_eq!(mse(&p, &t), 0.0);
    }

    #[test]
    fn test_mse_value() {
        let p = [0.0];
        let t = [1.0];
        // (0 - 1)^2 / 1 = 1.0
        assert!((mse(&p, &t) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_mse_derivative() {
        let p = [0.5];
        let t = [1.0];
        // 2 * (0.5 - 1.0) / 1 = -1.0
        let grad = mse_derivative(&p, &t);
        assert_eq!(grad.len(), 1);
        assert!((grad[0] - (-1.0)).abs() < 1e-6);
    }
}