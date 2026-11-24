# Agent Startup Checklist

When starting any new task, bug fix, or feature in this repository, follow these steps:

1. **Check Environment:**
   Run `cargo --version` (edition 2024 needs Rust ≥ 1.85). With Nix, `nix develop` (or direnv via
   `.envrc`) provides the full toolchain plus `make` and `node` for OpenSpec.
2. **Review OpenSpec:**
   Inspect `openspec/config.yaml`, run `openspec list` and `openspec list --specs`, and check
   `openspec/changes/` for active changes before starting.
   Workflow skills live in `.agents/skills/openspec-*` (propose → apply → archive). Any
   non-trivial change starts with a proposal under `openspec/changes/<name>/` and must pass
   `openspec validate <name> --strict` (or `make spec`).
3. **Know Where You Are in the Plan:**
   Read `README.md` (the 4-stage learning plan) and `.agents/shared/memory/project-status.md`
   (what is done, what is next). Keep the status memory current when a milestone lands.
4. **Understand Architectural Boundaries:**
   - Workspace root: `Cargo.toml` (members + shared release profile). One crate per milestone.
   - `xor/src/`: `activation.rs`, `loss.rs`, `layer.rs` (dense layer, forward/backward),
     `network.rs` (hidden + output layers, `train_step`), `random.rs` (xorshift RNG),
     `data.rs` (dataset), `main.rs` (training loop and evaluation).
5. **Run Verification Before & After:**
   Always run `make check` (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`,
   release build) before finishing any edit.
