# mnist — handwritten digits in pure Rust

Stage 1 of the learning plan, after `xor`: a dense neural network that learns to read
handwritten digits from the MNIST dataset (60,000 training and 10,000 test images, 28×28
greyscale). Standard library only: the IDX file parser, matrix maths, random numbers,
softmax, cross-entropy, and backpropagation are all written by hand.

## Run it

```sh
make mnist-data   # download and unpack the four MNIST files into data/mnist/ (once)
make mnist        # train with the defaults (release build)
make mnist ARGS="--epochs 2 --hidden 64"   # pass options
```

With the defaults it reaches about 98% test accuracy in 10 epochs, about 6 s per epoch on a
laptop. Loading the whole dataset as `f64` uses about 430 MB of RAM.

## Options

| Option | Meaning | Default |
|---|---|---|
| `--epochs <N>` | passes over the training set | 10 |
| `--lr <RATE>` | learning rate η | 0.5 |
| `--batch <N>` | mini-batch size B | 10 |
| `--hidden <N>` | hidden-layer neurons H | 128 |
| `--seed <N>` | random seed (same seed → identical run) | 42 |
| `--data <DIR>` | directory holding the four IDX files | `data/mnist` |
| `--help` | show usage | |

Invalid values (non-numeric, or 0 for epochs/lr/batch/hidden) print the usage and exit with
status 2. A missing dataset prints a hint to run `make mnist-data` and exits with status 1.

## The math in one screen

For one image x (784 pixels scaled to [0, 1]) with true digit `label`:

```
forward    a₁ = σ(W₁x + b₁)              hidden layer, H sigmoid neurons
           p  = softmax(W₂a₁ + b₂)       10 probabilities that sum to 1
loss       L  = −ln p[label]             cross-entropy
backward   δ₂ = p − y                    y is one-hot; softmax + cross-entropy simplify to this
           ∂L/∂W₂ = δ₂ a₁ᵀ   ∂L/∂b₂ = δ₂
           δ₁ = (W₂ᵀ δ₂) ⊙ a₁ ⊙ (1 − a₁)
           ∂L/∂W₁ = δ₁ xᵀ    ∂L/∂b₁ = δ₁
update     θ ← θ − η · (1/B) · Σ_batch ∂L/∂θ
```

The full derivation and the design choices are in
`openspec/changes/add-mnist-classifier/design.md`.

## Files

| File | Concept |
|---|---|
| `random.rs` | xorshift RNG and Fisher–Yates shuffle |
| `matrix.rs` | flat row-major matrix: `W x`, `Wᵀ v`, outer product |
| `activation.rs` | sigmoid and numerically stable softmax |
| `loss.rs` | cross-entropy |
| `network.rs` | parameters, forward, backward (gradient-checked), mini-batch update |
| `data.rs` | IDX parsing and validation |
| `metrics.rs` | accuracy and confusion matrix |
| `config.rs` | command-line options |
| `main.rs` | training loop and report |

Tests use only small synthetic data, so `cargo test` never needs the dataset.
