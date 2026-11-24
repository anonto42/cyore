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
