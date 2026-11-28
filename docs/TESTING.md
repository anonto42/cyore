# Testing

In a learning project, a wrong gradient can still produce a network that seems to learn, only worse. The tests exist to prove the math, not just to check that the code runs.

## The local gate
`make check` runs everything CI runs, in this order:

| Step | Command | Fails when |
|---|---|---|
| Format | `cargo fmt --all -- --check` | Any file is not `rustfmt`-clean |
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` | Clippy reports any warning |
| Tests | `cargo test --workspace` | Any unit test fails |
| Build | `cargo build --release -p $(CRATE)` | The optimised binary does not build |

`make spec` validates the OpenSpec specs and active changes with `--strict`. CI (`.github/workflows/ci.yml`) runs both inside the Nix dev shell on every push and pull request.

Tests never need downloaded data: the MNIST tests use small synthetic examples, so a fresh clone is green.

---

## What every numerical component must have

### 1. Numerical gradient check
Each backward pass is compared against central finite differences:

```
numeric  = (L(w + ε) − L(w − ε)) / 2ε          ε = 1e-5
analytic = what backprop computed
rel_err  = |analytic − numeric| / max(|analytic| + |numeric|, 1e-8)   must be < 1e-4
```

`xor`'s `test_gradients_match_finite_differences` nudges **every** weight and bias in the network one at a time. With learning rate 1, one `train_step` moves each parameter by exactly its gradient, so `analytic = before − after`. `mnist` checks its `backward` the same way.

### 2. Known values
Small hand-computed cases: `sigmoid(0) = 0.5`, `sigmoid'(0.5) = 0.25`, `mse([0], [1]) = 1`, `mse'([0.5], [1]) = −1`.

### 3. Ordering bugs
`test_backward_uses_pre_update_weights` uses a large learning rate, so any use of the *updated* weights in the returned gradient shows up immediately.

### 4. Reproducibility
- The same seed gives the same `Rng` sequence and identical initial weights.
- Seed 0 is remapped, so the generator never gets stuck emitting zeros.
- `mnist` also tests that the same seed gives the same per-epoch shuffle. Two full runs with the same seed were checked by hand to print byte-identical loss and accuracy.

### 5. Convergence
`test_learns_xor` trains with exactly the settings in `main` (2-4-1, seed 42, lr 1.0, 20,000 epochs) and requires all four truth-table rows to be correct after rounding.

### 6. Input validation (`mnist`)
The IDX loader rejects a wrong magic number, a truncated header or body, trailing bytes, a non-28×28 shape, mismatched image and label counts, and labels above 9, each with a clear error message. The CLI parser rejects unknown flags, missing values and non-positive numbers.

---

## Conventions
- Unit tests live next to the code in `#[cfg(test)] mod tests`.
- Compare floats with a tolerance (`(a - b).abs() < 1e-6`), never `==`, except for exact zeros.
- Training tests always use a fixed seed.
- `unwrap()` is fine in tests and nowhere else on user-input paths.
