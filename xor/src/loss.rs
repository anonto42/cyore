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