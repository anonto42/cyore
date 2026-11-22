/// Sigmoid: squashes any number into (0, 1)
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Derivative of sigmoid, written in terms of its OUTPUT.
/// Pass in s = sigmoid(x), not x itself.
pub fn sigmoid_derivative(s: f64) -> f64 {
    s * (1.0 - s)
}