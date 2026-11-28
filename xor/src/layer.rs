use crate::activation::{sigmoid, sigmoid_derivative};
use crate::random::Rng;

pub struct Layer {
    pub weights: Vec<Vec<f64>>, // weights[neuron][input]
    pub biases: Vec<f64>,       // one bias per neuron
    pub inputs: Vec<f64>,       // saved from the last forward pass (for backprop)
    pub outputs: Vec<f64>,      // saved from the last forward pass (for backprop)
}

// The loops below index with `j` (neuron) and `i` (input) on purpose, so the code reads like the
// math: z_j = b_j + sum_i w_ji * x_i. Iterator chains would hide that, so this lint is off here.
#[allow(clippy::needless_range_loop)]
impl Layer {
    pub fn new(n_inputs: usize, n_neurons: usize, rng: &mut Rng) -> Self {
        let mut weights = Vec::with_capacity(n_neurons);
        let mut biases = Vec::with_capacity(n_neurons);

        for _ in 0..n_neurons {
            let mut row = Vec::with_capacity(n_inputs);
            for _ in 0..n_inputs {
                row.push(rng.range(-1.0, 1.0));
            }
            weights.push(row);
            biases.push(rng.range(-1.0, 1.0));
        }

        Layer {
            weights,
            biases,
            inputs: vec![0.0; n_inputs],
            outputs: vec![0.0; n_neurons],
        }
    }

    pub fn forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        self.inputs = inputs.to_vec();

        for j in 0..self.weights.len() {
            let mut z = self.biases[j];
            for i in 0..inputs.len() {
                z += self.weights[j][i] * inputs[i];
            }
            self.outputs[j] = sigmoid(z);
        }

        self.outputs.clone()
    }

    /// output_grads[j] = how much the loss changes when output j changes.
    /// Updates this layer's weights and returns the gradients for the previous layer.
    pub fn backward(&mut self, output_grads: &[f64], learning_rate: f64) -> Vec<f64> {
        let mut input_grads = vec![0.0; self.inputs.len()];

        for j in 0..self.weights.len() {
            // chain rule through sigmoid: dL/dz = dL/dout * dout/dz
            let delta = output_grads[j] * sigmoid_derivative(self.outputs[j]);

            for i in 0..self.inputs.len() {
                // pass the gradient back using the OLD weight...
                input_grads[i] += delta * self.weights[j][i];
                // ...then update the weight
                self.weights[j][i] -= learning_rate * delta * self.inputs[i];
            }
            self.biases[j] -= learning_rate * delta;
        }

        input_grads
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_seed_same_parameters() {
        let a = Layer::new(3, 4, &mut Rng::new(42));
        let b = Layer::new(3, 4, &mut Rng::new(42));
        assert_eq!(a.weights, b.weights);
        assert_eq!(a.biases, b.biases);
    }

    #[test]
    fn test_initial_parameters_in_range() {
        let layer = Layer::new(5, 6, &mut Rng::new(1));
        for &w in layer.weights.iter().flatten().chain(&layer.biases) {
            assert!((-1.0..1.0).contains(&w));
        }
    }

    #[test]
    fn test_backward_uses_pre_update_weights() {
        let mut layer = Layer::new(3, 2, &mut Rng::new(42));
        layer.forward(&[0.5, -1.0, 2.0]);
        let old_weights = layer.weights.clone();
        let output_grads = [0.3, -0.7];

        // A large learning rate makes any use of the *updated* weights obvious.
        let input_grads = layer.backward(&output_grads, 10.0);

        for i in 0..3 {
            let mut expected = 0.0;
            for j in 0..2 {
                let delta = output_grads[j] * sigmoid_derivative(layer.outputs[j]);
                expected += delta * old_weights[j][i];
            }
            assert!((input_grads[i] - expected).abs() < 1e-12);
        }
    }
}
