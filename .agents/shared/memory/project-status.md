# Project Status & Design Decisions

Last updated: 2026-11-24

## Milestones (see README.md for the full plan)

| Stage | Milestone | Status |
|---|---|---|
| 1 | XOR network in pure Rust (`xor/`) | Done — converges to MSE ≈ 3e-5 in 20k epochs, seed 42 |
| 1 | MNIST digit recogniser | Next |
| 1 | Survey architectures (dense, CNN, RNN, transformer) | Not started |
| 2 | Burn: intent classifier + slot model, then tiny seq2seq | Not started |
| 3 | Rust agent with confirm-before-run and safety rules | Not started |
| 4 | int8 quantisation, `include_bytes!` weights, cross-compile | Release profile already set in root `Cargo.toml` |

## `xor` design

- Architecture 2 → 4 → 1, sigmoid everywhere, MSE loss, per-example SGD, learning rate 1.0.
- `Layer::forward` caches inputs and outputs for backprop, so `Network::predict` takes `&mut self`.
- `Layer::backward` accumulates the previous layer's gradient using the weights *before* updating
  them (correct ordering — keep it that way).
- RNG: xorshift64 in `random.rs`; seed 0 is remapped to a non-zero constant.

## Known follow-ups

- Add tests for `layer`, `network`, `random`: numerical gradient check and a convergence test.
- The final "successfully learned XOR" message is printed unconditionally.
- For MNIST: softmax + cross-entropy, mini-batches, flat weight storage, a prediction-only forward pass.
