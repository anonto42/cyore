# Design

## Context

`xor/` is a 2→4→1 sigmoid network with MSE loss, per-example SGD, weights stored as `Vec<Vec<f64>>`, and a `Layer::forward` that caches activations (so prediction needs `&mut self`). It is about 300 lines and has no dependencies. MNIST is about 15,000 times more data (60,000 × 784 inputs vs 4 × 2) with 10 output classes, so the XOR code works in principle but is too slow and uses the wrong output/loss pairing. See proposal.md for motivation and scope.

## Goals / Non-Goals

**Goals:**
- A readable, from-scratch implementation where each file maps to one concept, matching the style of `xor/`.
- Train 10 epochs on a normal laptop in a few minutes with `--release`.
- Make every new piece of math testable in isolation with small synthetic inputs, so `make check` never needs the real dataset.

**Non-Goals:**
- Peak accuracy (CNNs, ReLU/Adam, dropout, data augmentation can come later).
- Performance work beyond flat storage and avoiding per-example allocation (no threads, SIMD, or BLAS).

## The Math

Notation for one example: input x ∈ ℝ⁷⁸⁴, hidden size H (default 128), one-hot target y ∈ {0,1}¹⁰.

**Forward pass**
- Hidden: z₁ = W₁x + b₁ (W₁ is H×784), a₁ = σ(z₁), with σ(t) = 1 / (1 + e⁻ᵗ)
- Output: z₂ = W₂a₁ + b₂ (W₂ is 10×H), p = softmax(z₂), pₖ = e^(z₂ₖ − m) / Σⱼ e^(z₂ⱼ − m) where m = maxⱼ z₂ⱼ
  (subtracting m changes nothing mathematically but prevents overflow — spec: "Large inputs stay finite")

**Loss** — cross-entropy for one example: L = −Σₖ yₖ ln pₖ = −ln p_label.
Batch loss is the mean over the B examples in the mini-batch. Clamp p_label to ≥ 1e-15 before `ln` to avoid −∞.

**Backward pass** (per example, then averaged over the batch)
- Softmax + cross-entropy combine into a simple gradient: δ₂ = ∂L/∂z₂ = p − y
- ∂L/∂W₂ = δ₂ a₁ᵀ, ∂L/∂b₂ = δ₂
- Back through W₂: ∂L/∂a₁ = W₂ᵀ δ₂
- Through the sigmoid: δ₁ = (W₂ᵀ δ₂) ⊙ a₁ ⊙ (1 − a₁)
- ∂L/∂W₁ = δ₁ xᵀ, ∂L/∂b₁ = δ₁

**Update** (mini-batch SGD): θ ← θ − η · (1/B) Σ_batch ∂L/∂θ, applied once per batch after all B gradients are summed.

Why p − y matters for learning: with MSE + sigmoid output (as in XOR), the gradient is scaled by σ′, which is near 0 when the output is confidently wrong, so learning stalls. Cross-entropy cancels that factor.

## Decisions

1. **New crate, no shared library.** `mnist/` copies the small pieces it needs (xorshift `Rng`, `sigmoid`) instead of extracting a shared `nn` crate from `xor`.
   *Alternative:* a `nn/` library crate used by both. Rejected for now: it would change `xor`, and the right abstraction is clearer after a second example. Revisit when Stage 2 begins.

2. **Gradients separate from parameters.** A `Network` holds parameters; `forward(&self, x) -> Activations` returns the intermediate values instead of caching them in the layer, and `backward(&self, x, &Activations, label, &mut Gradients)` adds into a `Gradients` buffer. The optimiser step then applies `Gradients` and zeroes it.
   This makes `predict(&self)` free of side effects (fixes the XOR `&mut self` issue), lets the gradient check compare analytic vs numeric gradients without mutating weights, and makes mini-batching trivial (sum, then apply once).
   *Alternative:* keep XOR's `backward` that updates weights in place — incompatible with mini-batches and gradient checking.

3. **Flat row-major weights.** `Vec<f64>` of length rows×cols, indexed `w[r * cols + c]`, with a tiny `Matrix` struct (`rows`, `cols`, `data`) and two helpers: `matvec` (W x) and `matvec_transposed` (Wᵀ v). One contiguous allocation is cache-friendly and avoids 128 separate `Vec`s.
   *Alternative:* `Vec<Vec<f64>>` as in XOR — simpler to read but scattered in memory and slower on the 100k-weight hidden layer.

4. **Initialisation:** weights uniform in [−1/√fan_in, 1/√fan_in], biases 0. With XOR's [−1, 1] and 784 inputs, z₁ would have a standard deviation around 16, saturating every sigmoid (σ′ ≈ 0), and training would barely move.

5. **Data loading:** read the uncompressed IDX files with `std::fs::read` and parse the big-endian header with `u32::from_be_bytes`. Decompression happens once in `make mnist-data` (`curl` + `gunzip`), because the standard library has no gzip and a decoder is out of scope. Download from the commonly used mirror `https://storage.googleapis.com/cvdf-datasets/mnist/`; validity is checked by the loader's magic numbers and counts (60,000 / 10,000, 28×28) rather than by pinned checksums.
   Errors use a small `DataError` enum implementing `Display`; `main` prints it and exits with status 1 (no `panic`/`unwrap` on user input, per rust-standards).

6. **Shuffling:** Fisher–Yates over an index array with the seeded `Rng` at the start of each epoch, so examples are not reordered in memory and runs stay reproducible.

7. **Defaults:** H = 128, η = 0.5, B = 10, 10 epochs, seed 42, data dir `data/mnist`. This is a sigmoid-hidden + softmax/cross-entropy network known to reach roughly 96–98% on MNIST, which leaves margin over the 95% spec.

8. **CLI:** hand-rolled parsing over `std::env::args` for `--epochs --lr --batch --hidden --seed --data --help`. A dozen lines; no `clap`.

9. **Module layout** (one concept per file, mirroring `xor/`):
   `random.rs` (Rng + shuffle), `activation.rs` (sigmoid, softmax), `loss.rs` (cross-entropy), `matrix.rs`, `network.rs` (params, forward, backward, apply), `data.rs` (IDX parsing, `Dataset`), `metrics.rs` (accuracy, confusion matrix), `config.rs` (CLI), `main.rs` (training loop and report).

## Risks / Trade-offs

- [Training speed in pure scalar f64 — roughly 2×10¹⁰ floating-point operations per epoch] → `make mnist` always builds `--release`; the plan budgets about 10–30 s per epoch; flat storage and reusing `Activations`/`Gradients` buffers avoid per-example allocation.
- [Debug-mode tests would be far too slow on real data] → tests use synthetic inputs only (a tiny network for gradient checks, ten synthetic examples for overfitting); the 95% accuracy requirement is verified by a manual `make mnist` run recorded in tasks.md.
- [Mirror URL could disappear] → the URL lives in one Makefile variable; the loader's structural checks catch corrupt or wrong files.
- [Code duplicated between `xor` and `mnist` (`Rng`, `sigmoid`)] → accepted for teaching clarity; decision 1 records when to revisit.
- [Floating-point summation order affects results] → the order is fixed (single thread, seeded shuffle), so reproducibility holds on a given machine; results may differ by tiny amounts across CPUs, which the spec tolerates since it compares runs of the same binary.

## Migration Plan

Additive only: new workspace member and Makefile targets. Rollback is removing `mnist` from the workspace `members` and deleting the crate.
