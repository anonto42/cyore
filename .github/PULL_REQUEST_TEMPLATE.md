## Description
Provide a concise explanation of the changes introduced in this pull request and why they are needed.

**OpenSpec change:** `openspec/changes/<name>/` (or "n/a" for small fixes and docs)

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature or learning milestone
- [ ] Math / model change (forward pass, loss, gradients, training loop)
- [ ] Refactoring or teaching-clarity improvement
- [ ] Documentation update
- [ ] Build / CI improvement

## Architecture & Safety Checklist
- [ ] **From Scratch**: Stage 1 crates (`xor`, `mnist`) still depend only on the Rust standard library.
- [ ] **Correct Gradients**: Any forward/backward math change has a unit test or numerical gradient check.
- [ ] **Reproducibility**: Training still uses the seeded in-repo `Rng`, so the same seed gives the same result.
- [ ] **Human-in-the-Loop**: Agent code (Stage 3+) never runs a command without confirmation and enforces the allowlist/blocklist.
- [ ] **Minimal Footprint**: No new dependency, or the dependency is justified in `design.md`.
- [ ] **Testing**: `make check` passes cleanly (fmt, clippy `-D warnings`, tests, release build) and `make spec` validates.
- [ ] **Project Status**: `.agents/shared/memory/project-status.md` is updated if a milestone landed.
- [ ] **Git Hygiene**: Conventional commits used, no AI attribution lines included.
