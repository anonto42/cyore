# Contributing to Cyore

Thank you for your interest in contributing to Cyore! Cyore is a learning project, so clear explanations, extra tests, corrections to the math, and better comments are as welcome as new features.

Please read this guide before you open a pull request or propose a change to the learning plan.

---

## Code of Conduct

All contributors and participants agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## Architectural Invariants

Every contribution must keep Cyore's core rules. The full list is in [AGENTS.md](AGENTS.md):

1. **From Scratch in Stage 1**: The Stage 1 crates (`xor/`, `mnist/`) use only the Rust standard library. We do not accept ML, linear-algebra, or RNG crates there. Third-party crates such as Burn begin in Stage 2.
2. **Correct Gradients**: Any change to forward or backward math keeps or adds a test proving it: a unit test or a numerical gradient check (finite differences with ε ≈ 1e-5).
3. **Reproducibility**: Training uses the seeded in-repo `Rng`, so the same seed gives the same result.
4. **Human-in-the-Loop Execution**: From Stage 3 on, the agent never runs a command without showing it and getting explicit confirmation, and it always enforces the [safety rules](.agents/shared/rules/safety.md). We do not accept PRs that add an auto-run mode or weaken the blocklist.
5. **Minimal Footprint**: Every new dependency must be justified against the 5–30 MB binary / under 100 MB RAM target.
6. **Teaching Clarity**: Name things after the math (`delta`, `weights[neuron][input]`) and add comments that explain *why*, especially for each chain-rule step. See [Rust Standards](.agents/shared/rules/rust-standards.md).

---

## Development Setup

The project uses a Nix flake that provides Rust, `make`, `curl` and Node.js (for OpenSpec):

```bash
# Clone the repository
git clone https://github.com/anonto42/cyore.git
cd cyore

# Enter the Nix development shell
nix develop
# or with direnv:
direnv allow
```

Without Nix, install Rust ≥ 1.85 (edition 2024) with [rustup](https://rustup.rs), plus `make`. You only need Node.js to run `make spec`.

---

## Spec-Driven Changes

Non-trivial changes start as an [OpenSpec](openspec/) proposal before any code is written:

1. Check `openspec/changes/` for active changes that overlap with yours (`openspec list`).
2. Create `openspec/changes/<change-name>/` with `proposal.md`, `design.md`, `tasks.md`, and spec deltas. State the purpose, non-goals, invariants, and the learning-plan stage the change belongs to.
3. Write down the math (forward pass, loss, gradients) in `design.md` before writing the code.
4. Validate it with `openspec validate <change-name> --strict` (or `make spec`).

Small fixes, typos, and documentation improvements can skip the proposal.

---

## Testing & Verification

Before submitting any code change, make sure the full local gate passes:

```bash
# Run the workspace test suite
make test

# Full gate: format check, clippy -D warnings, tests, and release build
make check

# Validate OpenSpec specs and changes
make spec
```

Tests must not need downloaded data. For example, the MNIST tests use synthetic data, so `make check` stays green on a fresh clone. See the [Testing Guide](docs/TESTING.md).

---

## Commit Guidelines

We keep a clean, structured git history (see [Git Rules](.agents/shared/rules/git.md)):

1. **Conventional Commits**: Format commit messages as `type(scope): imperative summary`.
   - Types: `feat`, `fix`, `refactor`, `test`, `docs`, `style`, `build`, `ci`, `chore`.
   - Scopes: a crate (`xor`, `mnist`), `openspec`, `agents`, `docs`, `ci`.
   - Example: `test(xor): add numerical gradient check for the output layer`
2. **One Logical Change per Commit**: Keep crate code, specs, docs, and tooling in separate commits.
3. **No AI Attribution**: No `Co-Authored-By:` lines naming AI assistants, no "Generated with" footers, and no bot emojis in commits, PRs, issues, or comments.
4. **Author Identity**: Commit with your own git name and email.

---

## Submitting Pull Requests

1. Fork the repository and create a branch from `main`:
   ```bash
   git checkout -b feat/your-change-name
   ```
2. Keep modules small and single-purpose (activation, loss, layer, network, data, rng), with unit tests next to the code in `#[cfg(test)] mod tests`.
3. Check that `make check` and `make spec` pass cleanly.
4. Push your branch and open a pull request using the [PR Template](.github/PULL_REQUEST_TEMPLATE.md).
