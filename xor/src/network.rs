use crate::layer::Layer;
use crate::loss::{mse, mse_derivative};
use crate::random::Rng;

pub struct Network {
    hidden: Layer,
    output: Layer,
}

impl Network {
    pub fn new(n_inputs: usize, n_hidden: usize, n_outputs: usize, rng: &mut Rng) -> Self {
        Network {
            hidden: Layer::new(n_inputs, n_hidden, rng),
            output: Layer::new(n_hidden, n_outputs, rng),
        }
    }

    /// Forward pass through both layers
    pub fn predict(&mut self, inputs: &[f64]) -> Vec<f64> {
        let h = self.hidden.forward(inputs);
        self.output.forward(&h)
    }

    /// One training step on one example. Returns the loss before the update.
    pub fn train_step(&mut self, inputs: &[f64], targets: &[f64], learning_rate: f64) -> f64 {
        let prediction = self.predict(inputs);
        let loss = mse(&prediction, targets);

        let grad = mse_derivative(&prediction, targets);
        let hidden_grad = self.output.backward(&grad, learning_rate);
        self.hidden.backward(&hidden_grad, learning_rate);

        loss
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::xor_data;

    /// Every weight and bias in the network, in a fixed order, so a test can nudge one at a time.
    fn params_mut(net: &mut Network) -> Vec<&mut f64> {
        let mut params = Vec::new();
        for layer in [&mut net.hidden, &mut net.output] {
            params.extend(layer.weights.iter_mut().flatten());
            params.extend(layer.biases.iter_mut());
        }
        params
    }

    fn make_network() -> Network {
        Network::new(2, 3, 2, &mut Rng::new(3))
    }

    #[test]
    fn test_gradients_match_finite_differences() {
        let x = [0.3, -0.8];
        let t = [1.0, 0.0];

        // Analytic gradient: with learning_rate = 1, `w -= 1 * grad`, so grad = before - after.
        let mut net = make_network();
        let before: Vec<f64> = params_mut(&mut net).into_iter().map(|p| *p).collect();
        net.train_step(&x, &t, 1.0);
        let after: Vec<f64> = params_mut(&mut net).into_iter().map(|p| *p).collect();

        // Numerical gradient: central difference (L(w + eps) - L(w - eps)) / (2 * eps).
        let eps = 1e-5;
        let mut net = make_network();
        for k in 0..before.len() {
            *params_mut(&mut net)[k] += eps;
            let loss_plus = mse(&net.predict(&x), &t);
            *params_mut(&mut net)[k] -= 2.0 * eps;
            let loss_minus = mse(&net.predict(&x), &t);
            *params_mut(&mut net)[k] += eps;

            let numeric = (loss_plus - loss_minus) / (2.0 * eps);
            let analytic = before[k] - after[k];
            let rel_error = (analytic - numeric).abs() / (analytic.abs() + numeric.abs()).max(1e-8);
            assert!(
                rel_error < 1e-4,
                "param {k}: analytic {analytic}, numeric {numeric}"
            );
        }
    }

    #[test]
    fn test_learns_xor() {
        // Same settings as `main`: seed 42, 2-4-1, learning rate 1.0, 20,000 epochs.
        let (inputs, targets) = xor_data();
        let mut net = Network::new(2, 4, 1, &mut Rng::new(42));
        for _ in 0..20_000 {
            for (input, target) in inputs.iter().zip(&targets) {
                net.train_step(input, target, 1.0);
            }
        }
        for (input, target) in inputs.iter().zip(&targets) {
            let rounded = if net.predict(input)[0] >= 0.5 {
                1.0
            } else {
                0.0
            };
            assert_eq!(rounded, target[0], "wrong answer for {input:?}");
        }
    }
}
