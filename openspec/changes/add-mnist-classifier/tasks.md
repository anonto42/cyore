# Tasks

## 1. Scaffolding and dataset download

- [ ] 1.1 Create the `mnist/` crate (edition 2024, empty `[dependencies]`) and add it to workspace `members` in the root `Cargo.toml`; verify `cargo build -p mnist` succeeds
- [ ] 1.2 Add `data/` to `.gitignore`; verify `git check-ignore data/mnist/x` prints the path
- [ ] 1.3 Add a `MNIST_URL` variable and a `mnist-data` Makefile target that downloads the four `.gz` files with `curl` into `data/mnist/`, gunzips them, and skips files already present; add `curl` to `flake.nix`; verify running it twice downloads only once and leaves four IDX files
- [ ] 1.4 Add a `mnist` Makefile target that runs `cargo run --release -p mnist`, and list both new targets in `make help`; verify `make help` shows them

## 2. Building blocks (each with unit tests in the same file)

- [ ] 2.1 `random.rs`: copy the xorshift `Rng` from `xor`, add a Fisher–Yates `shuffle`; verify tests show same seed → same order, and that a shuffle is a permutation of the input
- [ ] 2.2 `matrix.rs`: flat row-major `Matrix` with `matvec` and `matvec_transposed`; verify tests against hand-computed 2×3 examples
- [ ] 2.3 `activation.rs`: `sigmoid` and numerically stable `softmax`; verify tests for sum = 1 (within 1e-9), non-negative outputs, and finite outputs for scores like `[1000, 999, -1000]`
- [ ] 2.4 `loss.rs`: cross-entropy for a predicted distribution and a label, clamping the probability at 1e-15; verify tests for a perfect prediction (≈ 0), a uniform 10-class prediction (= ln 10), and a zero probability (finite)

## 3. Dataset loading

- [ ] 3.1 `data.rs`: `DataError` enum with a `Display` message naming the file and the problem (missing, bad magic, truncated, count mismatch, label > 9)
- [ ] 3.2 Parse IDX images (magic 2051) into normalised `[0, 1]` values and IDX labels (magic 2049); verify tests on in-memory synthetic byte buffers for a valid 2-image 28×28 file, pixels 0 → 0.0 and 255 → 1.0, wrong magic, truncated body, and label 10
- [ ] 3.3 `load_mnist(dir)` loading the train and test pairs and checking that image and label counts match; verify a test that a mismatched pair returns the count-mismatch error, and that a missing directory returns an error mentioning `make mnist-data`

## 4. Network and training

- [ ] 4.1 `network.rs`: `Network` (W₁, b₁, W₂, b₂) with initialisation uniform in ±1/√fan_in and zero biases, `forward(&self)` returning `Activations`, and `predict(&self) -> usize`; verify tests that initial weights lie within bounds, that the same seed gives identical parameters, and that the output is a 10-value distribution
- [ ] 4.2 `backward` summing into a `Gradients` buffer, and `apply(&mut self, &mut Gradients, lr, batch_size)` that updates and zeroes it; verify with a numerical gradient check (central differences, ε = 1e-5) on a 4-input, 3-hidden, 10-output network over 3 examples, with every parameter's relative error below 1e-4
- [ ] 4.3 Overfit test: train a small network on 10 synthetic examples (one per class) for a fixed number of steps; verify all 10 are classified correctly
- [ ] 4.4 `metrics.rs`: `accuracy` and a 10×10 `confusion_matrix` with a pretty-printer; verify tests that entries sum to the number of examples and that a perfect classifier puts everything on the diagonal

## 5. Command-line program

- [ ] 5.1 `config.rs`: parse `--epochs --lr --batch --hidden --seed --data --help` with defaults (10, 0.5, 10, 128, 42, `data/mnist`), rejecting non-numeric and zero values; verify tests for defaults, overrides, `--batch 0`, and an unknown flag
- [ ] 5.2 `main.rs`: load data (print error + exit 1 on failure), shuffle each epoch, train in mini-batches, and after each epoch print epoch, average loss, test accuracy, and elapsed time; print the confusion matrix at the end; verify `cargo run -p mnist -- --data /nonexistent` exits non-zero with the download hint and no panic
- [ ] 5.3 Add a short `mnist/README.md` (what it does, the math summary, `make mnist-data && make mnist`, the options); verify the documented commands run as written

## 6. Integration checks

- [ ] 6.1 Run `make mnist-data && make mnist`; verify the final test accuracy is ≥ 95% and record the accuracy and time per epoch in `.agents/shared/memory/project-status.md`
- [ ] 6.2 Run `make mnist` with `--epochs 2` twice with the same seed; verify both runs print identical losses and accuracies
- [ ] 6.3 Update `AGENTS.md` (workspace members) and mark the MNIST milestone done in `project-status.md`; run `make check` and `make spec` and verify both pass

## Workflow follow-up

- Archive the change with `openspec archive add-mnist-classifier` after review, which creates `openspec/specs/mnist-dataset` and `openspec/specs/mnist-classifier`.
- Verify `make spec` passes after archive.
