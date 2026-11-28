# Project Status & Design Decisions

Last updated: 2026-11-28

## Milestones (see README.md for the full plan)

| Stage | Milestone | Status |
|---|---|---|
| 1 | XOR network in pure Rust (`xor/`) | Done — converges to MSE ≈ 3e-5 in 20k epochs, seed 42 |
| 1 | MNIST digit recogniser (`mnist/`) | Done — 97.96% test accuracy after 10 epochs (defaults, seed 42), ≈6 s/epoch release on a 12-thread laptop, peak RAM ≈430 MB |
| 1 | Survey architectures (dense, CNN, RNN, transformer) | Next (study, no code) |
| 2 | Burn: intent classifier + slot model, then tiny seq2seq | Not started |
| 3 | Rust agent with confirm-before-run and safety rules | Not started |
| 4 | int8 quantisation, `include_bytes!` weights, cross-compile | Release profile already set in root `Cargo.toml` |

## `xor` design

- Architecture 2 → 4 → 1, sigmoid everywhere, MSE loss, per-example SGD, learning rate 1.0.
- `Layer::forward` caches inputs and outputs for backprop, so `Network::predict` takes `&mut self`.
- `Layer::backward` accumulates the previous layer's gradient using the weights *before* updating
  them (correct ordering — keep it that way).
- RNG: xorshift64 in `random.rs`; seed 0 is remapped to a non-zero constant.

- Tests cover every spec scenario: RNG seeding, layer init, pre-update-weight backprop, a numerical
  gradient check of the whole network, and a 20k-epoch convergence test.

## `mnist` design

- 784 → 128 (sigmoid) → 10 (softmax), cross-entropy, mini-batch SGD (B = 10, η = 0.5), 10 epochs.
- `Network` holds only parameters; `forward_into` fills `Activations`, `backward` *adds* into
  `Gradients`, `apply` steps and zeroes them. `predict(&self)` has no side effects.
- Weights uniform in ±1/√fan_in, biases 0 (XOR's ±1 would saturate 784-input sigmoids).
- Flat row-major `Matrix` (`matvec`, `matvec_transposed`, `add_outer`).
- Same seed → byte-identical per-epoch loss and accuracy (verified with two `--epochs 2` runs).
- Dataset: `make mnist-data` downloads from `storage.googleapis.com/cvdf-datasets/mnist` into
  git-ignored `data/mnist/`; the loader validates magic, length, 28×28 shape, counts, labels.

## Known follow-ups

- `mnist` keeps all pixels as `f64` (≈430 MB). Storing `u8` and normalising per example would cut
  that to ≈60 MB if memory ever matters for training.
- Archive the `add-mnist-classifier` OpenSpec change after review (`openspec archive`).
