# xor-network Specification

## Purpose

The `xor` crate is a dependency-free, two-layer neural network that learns the XOR truth table using a hand-written forward pass, mean squared error loss, and backpropagation, as the first milestone of the learning plan.

## Requirements

### Requirement: Standard-library only
The `xor` crate SHALL depend only on the Rust standard library, including its own random number generator.

#### Scenario: Dependency audit
- **WHEN** `xor/Cargo.toml` is inspected
- **THEN** its `[dependencies]` table is empty

### Requirement: Reproducible initialisation
The network SHALL initialise every weight and bias uniformly in [-1, 1) from a seeded xorshift generator, and the same seed MUST produce identical parameters.

#### Scenario: Same seed
- **WHEN** two networks are created with the same seed and layer sizes
- **THEN** their weights and biases are identical

#### Scenario: Zero seed
- **WHEN** the generator is seeded with 0
- **THEN** it substitutes a non-zero internal state so it never emits only zeros

### Requirement: Correct backpropagation
Each layer's backward pass SHALL apply the chain rule through the sigmoid, SHALL compute the gradient for the previous layer using the weights from before the update, and SHALL then update weights and biases by gradient descent.

#### Scenario: Gradient for previous layer
- **WHEN** a layer runs its backward pass with a given output gradient
- **THEN** the returned input gradient equals the sum over neurons of delta multiplied by the pre-update weight

### Requirement: Learns XOR
Training the 2-4-1 network with seed 42, learning rate 1.0, and 20,000 epochs of per-example updates SHALL classify all four XOR inputs correctly when outputs are rounded at 0.5.

#### Scenario: Truth table after training
- **WHEN** training completes and each of the four inputs is evaluated
- **THEN** the rounded prediction equals the XOR target for every input
