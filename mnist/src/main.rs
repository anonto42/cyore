// Train a two-layer network on MNIST and report test accuracy after every epoch.
// The math is written out in openspec/changes/add-mnist-classifier/design.md.

mod activation;
mod config;
mod data;
mod loss;
mod matrix;
mod metrics;
mod network;
mod random;

use std::process::ExitCode;
use std::time::Instant;

use config::{Command, USAGE, parse_args};
use data::{Dataset, NUM_CLASSES, load_mnist};
use loss::cross_entropy;
use metrics::{ConfusionMatrix, accuracy};
use network::Network;
use random::Rng;

/// Predict every example in `data` (forward passes only, weights never change).
fn predict_all(net: &Network, data: &Dataset) -> Vec<usize> {
    (0..data.len())
        .map(|i| net.predict(data.image(i)))
        .collect()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let config = match parse_args(&args) {
        Ok(Command::Train(config)) => config,
        Ok(Command::Help) => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };

    println!("=== MNIST Digit Classifier in Pure Rust ===\n");
    println!("Loading dataset from {} ...", config.data_dir.display());
    let (train, test) = match load_mnist(&config.data_dir) {
        Ok(sets) => sets,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    println!(
        "Loaded {} training and {} test images.\n",
        train.len(),
        test.len()
    );

    let mut rng = Rng::new(config.seed);
    let mut net = Network::new(train.n_inputs, config.hidden, NUM_CLASSES, &mut rng);
    let mut grads = net.new_gradients();
    let mut act = net.new_activations();
    let test_labels: Vec<usize> = (0..test.len()).map(|i| test.label(i)).collect();

    println!(
        "Network {}-{}-{} | epochs {} | learning rate {} | batch {} | seed {}\n",
        train.n_inputs,
        config.hidden,
        NUM_CLASSES,
        config.epochs,
        config.learning_rate,
        config.batch_size,
        config.seed
    );

    // We shuffle this list of indices each epoch instead of moving the images around.
    let mut order: Vec<usize> = (0..train.len()).collect();
    let start = Instant::now();
    let mut predictions = Vec::new();

    for epoch in 1..=config.epochs {
        let epoch_start = Instant::now();
        rng.shuffle(&mut order);

        let mut total_loss = 0.0;
        for batch in order.chunks(config.batch_size) {
            // Sum the gradients of every example in the batch, then take one step.
            for &i in batch {
                let (x, label) = (train.image(i), train.label(i));
                net.forward_into(x, &mut act);
                total_loss += cross_entropy(&act.output, label);
                net.backward(x, &act, label, &mut grads);
            }
            net.apply(&mut grads, config.learning_rate, batch.len());
        }

        // Evaluate on the test set, which is never used for weight updates.
        predictions = predict_all(&net, &test);
        println!(
            "Epoch {:>3}/{} | loss {:.6} | test accuracy {:>6.2}% | {:.1}s",
            epoch,
            config.epochs,
            total_loss / train.len() as f64,
            100.0 * accuracy(&predictions, &test_labels),
            epoch_start.elapsed().as_secs_f64()
        );
    }

    let confusion = ConfusionMatrix::new(&predictions, &test_labels, NUM_CLASSES);
    println!("\n=== Confusion matrix on the test set (rows: true digit, columns: predicted) ===");
    print!("{confusion}");
    println!(
        "\nFinal test accuracy: {:.2}% ({} images) in {:.1}s total.",
        100.0 * accuracy(&predictions, &test_labels),
        confusion.total(),
        start.elapsed().as_secs_f64()
    );
    ExitCode::SUCCESS
}
