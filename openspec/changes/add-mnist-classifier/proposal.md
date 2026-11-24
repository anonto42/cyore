# Proposal

## Why

The XOR milestone proved the forward pass and backpropagation on 4 hand-written examples. Stage 1 of the learning plan (README.md) next calls for recognising handwritten digits (MNIST): training on real data — 60,000 images, 10 classes — so the core ideas carry over to the Stage 2 intent classifier, which is the same kind of multi-class classifier.

## What Changes

- Add a new workspace crate `mnist/`, standard library only, with a binary that trains a dense network on the MNIST training set and reports accuracy on the 10,000-image test set.
- Read the MNIST IDX binary format directly, with validation and clear error messages for missing or corrupt files.
- Introduce the concepts XOR did not need: softmax output with cross-entropy loss, mini-batch gradient descent, per-epoch shuffling, input normalisation, scaled weight initialisation, and test-set evaluation (accuracy and a confusion matrix).
- Add a `make mnist-data` target that downloads and decompresses the dataset into `data/mnist/` (git-ignored), and `make mnist` to train.
- Update `AGENTS.md` and `.agents/shared/memory/project-status.md` to record the milestone.

### Non-goals

- No third-party crates (no BLAS, `rand`, `flate2`, or ML frameworks); those are Stage 2.
- No convolutional layers, GPU, multithreading, or SIMD.
- No saving or loading of trained weights (Burn covers this in Stage 2).
- No shared `nn` library crate extracted from `xor`; `xor` stays unchanged.
- No Docker image for `mnist` (it needs the dataset at runtime).

### Invariants

- `xor` behaviour and its spec are untouched.
- Every backward pass is covered by a numerical gradient check.
- The same seed and hyperparameters produce the same training run.
- `make check` stays green without the dataset present (tests use synthetic data).

## Capabilities

### New Capabilities
- `mnist-dataset`: obtaining the MNIST files and loading the IDX image and label format into normalised examples, with validation of structure and counts.
- `mnist-classifier`: training a digit classifier on MNIST and evaluating it, including reproducibility, configurable hyperparameters, and reported metrics.

### Modified Capabilities

None.

## Impact

- New: `mnist/` crate (workspace member in root `Cargo.toml`), `data/` directory (ignored).
- Changed: root `Cargo.toml` (members), `Cargo.lock`, `Makefile` (two targets), `.gitignore`, `AGENTS.md`, `.agents/shared/memory/project-status.md`.
- Build tooling: `make mnist-data` needs `curl` and `gunzip` on the host (both in the Nix dev shell's base system; `curl` added to `flake.nix`).
- No new dependencies; no effect on the `xor` binary or the Docker image.
