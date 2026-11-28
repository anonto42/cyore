# Spec Delta

## Purpose

Trains a dependency-free dense neural network to recognise handwritten digits from MNIST and reports how well it generalises to unseen test images, as the real-data milestone of Stage 1.

## ADDED Requirements

### Requirement: Standard-library only
The `mnist` crate SHALL depend only on the Rust standard library.

#### Scenario: Dependency audit
- **WHEN** `mnist/Cargo.toml` is inspected
- **THEN** its `[dependencies]` table is empty

### Requirement: Probability output
For any input image the network SHALL output 10 values, one per digit, each in [0, 1] and summing to 1, and the predicted digit SHALL be the one with the highest value.

#### Scenario: Output is a distribution
- **WHEN** any 784-value input is passed through the network
- **THEN** it returns 10 non-negative values whose sum is within 1e-9 of 1

#### Scenario: Large inputs stay finite
- **WHEN** the output layer's raw scores are very large (e.g. 1000)
- **THEN** every output value is finite (no NaN or infinity)

### Requirement: Correct gradients
The gradients the network computes for every weight and bias SHALL match a finite-difference estimate of the loss gradient.

#### Scenario: Gradient check
- **WHEN** the analytic gradients of a small network are compared with central finite differences (ε = 1e-5) on a few examples
- **THEN** the relative error for every parameter is below 1e-4

### Requirement: Learns from examples
Training SHALL reduce the loss on the data it is trained on.

#### Scenario: Overfit a tiny set
- **WHEN** the network is trained repeatedly on a small fixed synthetic set (e.g. 10 examples, one per class)
- **THEN** it classifies every example in that set correctly

### Requirement: Test accuracy
With default settings, training on the 60,000 training images SHALL reach at least 95% accuracy on the 10,000 test images, which are never used for weight updates.

#### Scenario: Default training run
- **WHEN** `make mnist` runs with the default hyperparameters and the dataset present
- **THEN** the final reported test accuracy is at least 95%

### Requirement: Reproducible training
Two training runs with the same seed and hyperparameters SHALL produce identical results.

#### Scenario: Same seed twice
- **WHEN** training runs twice with the same seed, data, and hyperparameters
- **THEN** both runs report identical per-epoch losses and accuracies

### Requirement: Configurable hyperparameters
The program SHALL accept command-line options for epochs, learning rate, batch size, hidden-layer size, seed, and data directory, SHALL use documented defaults for omitted options, and SHALL reject invalid values with a usage message and a non-zero exit status.

#### Scenario: Override epochs
- **WHEN** the program runs with `--epochs 1`
- **THEN** it trains for exactly one epoch

#### Scenario: Invalid value
- **WHEN** an option has a non-numeric or zero value (e.g. `--batch 0`)
- **THEN** the program prints usage help and exits with a non-zero status without training

### Requirement: Progress and evaluation report
The program SHALL print, after each epoch, the average training loss and the test accuracy, and at the end SHALL print a 10×10 confusion matrix of true digit versus predicted digit on the test set.

#### Scenario: Per-epoch line
- **WHEN** an epoch finishes
- **THEN** a line with the epoch number, average training loss, test accuracy, and elapsed time is printed

#### Scenario: Confusion matrix
- **WHEN** training completes
- **THEN** a confusion matrix is printed whose entries sum to the number of test images
