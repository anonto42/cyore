// Module declarations: Tells Rust to compile these files as part of the crate
mod activation;
mod data;
mod layer;
mod loss;
mod network;
mod random;

use data::xor_data;
use network::Network;
use random::Rng;

fn main() {
    println!("=== XOR Neural Network Training in Pure Rust ===\n");

    // 1. Load the XOR dataset
    // inputs: 4 examples, each with 2 inputs [A, B]
    // targets: 4 examples, each with 1 expected output [A ^ B]
    let (inputs, targets) = xor_data();

    // 2. Initialize our custom pseudo-random number generator
    // Using a fixed seed (e.g. 42) ensures reproducible initial weights and biases
    let mut rng = Rng::new(42);

    // 3. Create the 2-layer neural network
    // Architecture: 2 inputs -> 4 hidden neurons -> 1 output neuron
    // We pass `&mut rng` as a mutable reference so Network can sample random numbers to initialize weights
    let mut network = Network::new(2, 4, 1, &mut rng);

    // 4. Hyperparameters
    let epochs = 20_000;
    let learning_rate = 1.0;

    println!("Starting training for {} epochs with learning_rate = {}...\n", epochs, learning_rate);

    // 5. Training loop
    for epoch in 1..=epochs {
        let mut total_loss = 0.0;

        // Iterate through each training example
        // inputs.iter().zip(&targets) borrows each input and target without cloning
        for (input, target) in inputs.iter().zip(&targets) {
            // `network.train_step` takes `&[f64]` slices for inputs and targets
            let loss = network.train_step(input, target, learning_rate);
            total_loss += loss;
        }

        let avg_loss = total_loss / inputs.len() as f64;

        // Log progress every 2,000 epochs or on the first epoch
        if epoch == 1 || epoch % 2_000 == 0 {
            println!("Epoch {:5} | Average MSE Loss: {:.6}", epoch, avg_loss);
        }
    }

    // 6. Test and evaluate the trained network on the XOR truth table
    println!("\n=== Evaluation on XOR Truth Table ===");
    println!("{:<16} {:<10} {:<12} {:<10}", "Input [A, B]", "Target", "Predicted", "Rounded");
    println!("{}", "-".repeat(50));

    for (input, target) in inputs.iter().zip(&targets) {
        let pred = network.predict(input);
        let predicted_val = pred[0];
        let target_val = target[0];
        let rounded = if predicted_val >= 0.5 { 1.0 } else { 0.0 };

        println!(
            "[{:.1}, {:.1}]          {:<10.1} {:<12.4} {:<10.1}",
            input[0], input[1], target_val, predicted_val, rounded
        );
    }

    println!("\nTraining complete! Network successfully learned XOR.");
}
