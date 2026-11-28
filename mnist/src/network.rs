// A two-layer dense network for classification:
//
//   x (784) ──W₁,b₁──> z₁ ──σ──> a₁ (H) ──W₂,b₂──> z₂ ──softmax──> p (10)
//
// Unlike `xor`, the network only *holds parameters*. `forward` returns the intermediate values
// (`Activations`) instead of caching them, and `backward` adds into a separate `Gradients`
// buffer. That keeps `predict` free of side effects and makes mini-batches simple: sum the
// gradients of B examples, then `apply` them once (see design.md, decision 2).

use crate::activation::{sigmoid, sigmoid_derivative, softmax};
use crate::matrix::Matrix;
use crate::random::Rng;

pub struct Network {
    pub w1: Matrix,   // H × n_inputs
    pub b1: Vec<f64>, // H
    pub w2: Matrix,   // n_outputs × H
    pub b2: Vec<f64>, // n_outputs
}

/// The values computed by one forward pass, needed again by the backward pass.
pub struct Activations {
    pub hidden: Vec<f64>, // a₁ = σ(W₁x + b₁)
    pub scores: Vec<f64>, // z₂ = W₂a₁ + b₂ (raw scores, before softmax)
    pub output: Vec<f64>, // p = softmax(z₂)
}

/// Summed gradients ∂L/∂θ for every parameter, with the same shapes as `Network`.
pub struct Gradients {
    pub w1: Matrix,
    pub b1: Vec<f64>,
    pub w2: Matrix,
    pub b2: Vec<f64>,
    // Scratch space for δ₂ and δ₁, reused so `backward` does not allocate per example.
    delta_output: Vec<f64>,
    delta_hidden: Vec<f64>,
}

/// Index of the largest value (the predicted class).
pub fn argmax(values: &[f64]) -> usize {
    let mut best = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > values[best] {
            best = i;
        }
    }
    best
}

/// A matrix with every entry uniform in [−1/√fan_in, 1/√fan_in].
///
/// Why the 1/√fan_in scale: z = Σ wᵢxᵢ adds up fan_in terms. With XOR's [−1, 1] weights and
/// 784 inputs, z would be so large that every sigmoid saturates (σ′ ≈ 0) and nothing learns.
fn scaled_uniform(rows: usize, cols: usize, rng: &mut Rng) -> Matrix {
    let limit = 1.0 / (cols as f64).sqrt();
    let mut m = Matrix::zeros(rows, cols);
    for w in &mut m.data {
        *w = rng.range(-limit, limit);
    }
    m
}

impl Network {
    pub fn new(n_inputs: usize, n_hidden: usize, n_outputs: usize, rng: &mut Rng) -> Self {
        Network {
            w1: scaled_uniform(n_hidden, n_inputs, rng),
            b1: vec![0.0; n_hidden],
            w2: scaled_uniform(n_outputs, n_hidden, rng),
            b2: vec![0.0; n_outputs],
        }
    }

    pub fn n_hidden(&self) -> usize {
        self.w1.rows
    }

    pub fn n_outputs(&self) -> usize {
        self.w2.rows
    }

    /// Empty buffers of the right size for `forward_into`.
    pub fn new_activations(&self) -> Activations {
        Activations {
            hidden: vec![0.0; self.n_hidden()],
            scores: vec![0.0; self.n_outputs()],
            output: vec![0.0; self.n_outputs()],
        }
    }

    /// A zeroed gradient buffer of the right size for `backward`.
    pub fn new_gradients(&self) -> Gradients {
        Gradients {
            w1: Matrix::zeros(self.w1.rows, self.w1.cols),
            b1: vec![0.0; self.b1.len()],
            w2: Matrix::zeros(self.w2.rows, self.w2.cols),
            b2: vec![0.0; self.b2.len()],
            delta_output: vec![0.0; self.n_outputs()],
            delta_hidden: vec![0.0; self.n_hidden()],
        }
    }

    /// Forward pass, writing into reusable buffers (no allocation).
    pub fn forward_into(&self, x: &[f64], act: &mut Activations) {
        // Hidden layer: a₁ = σ(W₁x + b₁)
        self.w1.matvec(x, &mut act.hidden);
        for (a, b) in act.hidden.iter_mut().zip(&self.b1) {
            *a = sigmoid(*a + b);
        }
        // Output layer: z₂ = W₂a₁ + b₂, then p = softmax(z₂)
        self.w2.matvec(&act.hidden, &mut act.scores);
        for (z, b) in act.scores.iter_mut().zip(&self.b2) {
            *z += b;
        }
        softmax(&act.scores, &mut act.output);
    }

    /// Forward pass returning fresh buffers. Convenient, but allocates.
    pub fn forward(&self, x: &[f64]) -> Activations {
        let mut act = self.new_activations();
        self.forward_into(x, &mut act);
        act
    }

    /// The predicted digit: the class with the highest probability.
    pub fn predict(&self, x: &[f64]) -> usize {
        argmax(&self.forward(x).output)
    }

    /// Cross-entropy loss for one example (the training loop computes it inline from the
    /// forward pass it already did; tests use this for the gradient check).
    #[cfg(test)]
    pub fn loss(&self, x: &[f64], label: usize) -> f64 {
        crate::loss::cross_entropy(&self.forward(x).output, label)
    }

    /// Backward pass for one example: ADDS ∂L/∂θ into `grads` (it does not overwrite them),
    /// so calling it for every example in a batch sums their gradients.
    /// `act` must come from `forward_into(x, ..)` with the same `x`.
    pub fn backward(&self, x: &[f64], act: &Activations, label: usize, grads: &mut Gradients) {
        // Output layer. Softmax + cross-entropy combine into a very simple gradient:
        //   δ₂ = ∂L/∂z₂ = p − y   (y is one-hot: 1 at `label`, 0 elsewhere)
        for (k, (d, &p)) in grads.delta_output.iter_mut().zip(&act.output).enumerate() {
            *d = if k == label { p - 1.0 } else { p };
        }
        // ∂L/∂W₂ = δ₂ a₁ᵀ,  ∂L/∂b₂ = δ₂
        grads.w2.add_outer(&grads.delta_output, &act.hidden);
        for (g, d) in grads.b2.iter_mut().zip(&grads.delta_output) {
            *g += d;
        }

        // Hidden layer. Send the error back through W₂ (using the current, not-yet-updated
        // weights), then through the sigmoid:
        //   δ₁ = (W₂ᵀ δ₂) ⊙ σ′  where σ′ = a₁(1 − a₁)
        self.w2
            .matvec_transposed(&grads.delta_output, &mut grads.delta_hidden);
        for (d, &a) in grads.delta_hidden.iter_mut().zip(&act.hidden) {
            *d *= sigmoid_derivative(a);
        }
        // ∂L/∂W₁ = δ₁ xᵀ,  ∂L/∂b₁ = δ₁
        grads.w1.add_outer(&grads.delta_hidden, x);
        for (g, d) in grads.b1.iter_mut().zip(&grads.delta_hidden) {
            *g += d;
        }
    }

    /// Mini-batch gradient descent step: θ ← θ − η · (1/B) · Σ ∂L/∂θ.
    /// `grads` holds the sum over `batch_size` examples; it is zeroed afterwards, ready for
    /// the next batch.
    pub fn apply(&mut self, grads: &mut Gradients, learning_rate: f64, batch_size: usize) {
        let step = learning_rate / batch_size as f64;
        let pairs = [
            (&mut self.w1.data, &mut grads.w1.data),
            (&mut self.b1, &mut grads.b1),
            (&mut self.w2.data, &mut grads.w2.data),
            (&mut self.b2, &mut grads.b2),
        ];
        for (params, grad) in pairs {
            for (p, g) in params.iter_mut().zip(grad.iter_mut()) {
                *p -= step * *g;
                *g = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn random_inputs(n: usize, len: usize, rng: &mut Rng) -> Vec<Vec<f64>> {
        (0..n)
            .map(|_| (0..len).map(|_| rng.next_f64()).collect())
            .collect()
    }

    #[test]
    fn test_initial_weights_within_bounds() {
        let net = Network::new(784, 32, 10, &mut Rng::new(42));
        let limit1 = 1.0 / 784f64.sqrt();
        let limit2 = 1.0 / 32f64.sqrt();
        assert!(net.w1.data.iter().all(|w| w.abs() <= limit1));
        assert!(net.w2.data.iter().all(|w| w.abs() <= limit2));
        assert!(net.b1.iter().chain(&net.b2).all(|&b| b == 0.0));
    }

    #[test]
    fn test_same_seed_same_parameters() {
        let a = Network::new(20, 8, 10, &mut Rng::new(5));
        let b = Network::new(20, 8, 10, &mut Rng::new(5));
        assert_eq!(a.w1, b.w1);
        assert_eq!(a.w2, b.w2);
        assert_eq!(a.b1, b.b1);
        assert_eq!(a.b2, b.b2);
    }

    #[test]
    fn test_output_is_distribution() {
        let mut rng = Rng::new(1);
        let net = Network::new(784, 16, 10, &mut rng);
        for x in random_inputs(5, 784, &mut rng) {
            let p = net.forward(&x).output;
            assert_eq!(p.len(), 10);
            assert!(p.iter().all(|&v| v >= 0.0));
            assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-9);
            assert!(net.predict(&x) < 10);
        }
    }

    #[test]
    fn test_large_scores_stay_finite() {
        // Force huge output scores through a big bias.
        let mut net = Network::new(4, 3, 10, &mut Rng::new(1));
        net.b2[2] = 1000.0;
        net.b2[3] = 999.0;
        let p = net.forward(&[0.5; 4]).output;
        assert!(p.iter().all(|v| v.is_finite()));
        assert_eq!(argmax(&p), 2);
    }

    #[test]
    fn test_argmax() {
        assert_eq!(argmax(&[0.1, 0.7, 0.2]), 1);
        assert_eq!(argmax(&[0.5, 0.5]), 0); // ties go to the first
    }

    /// Mutable views of the four parameter groups, so a test can nudge one value at a time.
    fn param_group(net: &mut Network, group: usize) -> &mut [f64] {
        match group {
            0 => &mut net.w1.data,
            1 => &mut net.b1,
            2 => &mut net.w2.data,
            _ => &mut net.b2,
        }
    }

    fn grad_group(grads: &Gradients, group: usize) -> &[f64] {
        match group {
            0 => &grads.w1.data,
            1 => &grads.b1,
            2 => &grads.w2.data,
            _ => &grads.b2,
        }
    }

    #[test]
    fn test_gradients_match_finite_differences() {
        let mut rng = Rng::new(7);
        let mut net = Network::new(4, 3, 10, &mut rng);
        // Non-zero biases so their gradients are exercised away from the symmetric start.
        for b in net.b1.iter_mut().chain(net.b2.iter_mut()) {
            *b = rng.range(-0.5, 0.5);
        }
        let inputs = random_inputs(3, 4, &mut rng);
        let labels = [2, 7, 2];
        let total_loss = |net: &Network| -> f64 {
            inputs
                .iter()
                .zip(labels)
                .map(|(x, label)| net.loss(x, label))
                .sum()
        };

        // Analytic: sum backward() over the 3 examples.
        let mut grads = net.new_gradients();
        for (x, &label) in inputs.iter().zip(&labels) {
            let act = net.forward(x);
            net.backward(x, &act, label, &mut grads);
        }

        // Numerical: central differences, (L(θ + ε) − L(θ − ε)) / 2ε, one parameter at a time.
        let eps = 1e-5;
        for group in 0..4 {
            for i in 0..grad_group(&grads, group).len() {
                let original = param_group(&mut net, group)[i];
                param_group(&mut net, group)[i] = original + eps;
                let loss_plus = total_loss(&net);
                param_group(&mut net, group)[i] = original - eps;
                let loss_minus = total_loss(&net);
                param_group(&mut net, group)[i] = original;

                let numeric = (loss_plus - loss_minus) / (2.0 * eps);
                let analytic = grad_group(&grads, group)[i];
                let rel_error =
                    (analytic - numeric).abs() / (analytic.abs() + numeric.abs()).max(1e-8);
                assert!(
                    rel_error < 1e-4,
                    "group {group} index {i}: analytic {analytic}, numeric {numeric}, rel {rel_error}"
                );
            }
        }
    }

    #[test]
    fn test_apply_updates_and_zeroes() {
        let mut net = Network::new(2, 2, 10, &mut Rng::new(3));
        let mut grads = net.new_gradients();
        grads.b2[0] = 4.0;
        let before = net.b2[0];
        net.apply(&mut grads, 0.5, 2); // step = 0.5 / 2 = 0.25 → change of 0.25 · 4 = 1
        assert!((net.b2[0] - (before - 1.0)).abs() < 1e-12);
        assert_eq!(grads.b2[0], 0.0);
    }

    #[test]
    fn test_overfits_tiny_set() {
        // Ten random 16-pixel "images", one per class. A network that learns at all should
        // memorise them.
        let mut rng = Rng::new(42);
        let inputs = random_inputs(10, 16, &mut rng);
        let mut net = Network::new(16, 16, 10, &mut rng);
        let mut grads = net.new_gradients();
        let mut act = net.new_activations();

        let loss_before: f64 = (0..10).map(|k| net.loss(&inputs[k], k)).sum();
        for _ in 0..500 {
            for (label, x) in inputs.iter().enumerate() {
                net.forward_into(x, &mut act);
                net.backward(x, &act, label, &mut grads);
            }
            net.apply(&mut grads, 1.0, inputs.len());
        }
        let loss_after: f64 = (0..10).map(|k| net.loss(&inputs[k], k)).sum();

        assert!(loss_after < loss_before);
        for (label, x) in inputs.iter().enumerate() {
            assert_eq!(net.predict(x), label, "example {label} misclassified");
        }
    }
}
