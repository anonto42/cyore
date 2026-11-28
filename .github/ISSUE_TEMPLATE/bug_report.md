---
name: Bug Report
about: Create a report to help us improve Cyore
title: "[BUG] "
labels: bug
assignees: ''
---

**Describe the Bug**
A clear and concise description of what the bug is.

**Crate / Stage**
- Crate: [e.g. `xor`, `mnist`]
- Learning-plan stage: [e.g. Stage 1]

**Environment**
- Operating System & Architecture: [e.g. Debian 13 x86_64, Raspberry Pi OS aarch64, Windows 11]
- Rust Version: [output of `rustc --version`]
- Dev Shell: [Nix flake / rustup / Docker]
- Git SHA: [output of `git rev-parse --short HEAD`]

**Steps to Reproduce**
1. Run command: [e.g. `make run CRATE=xor`, `make mnist ARGS="--epochs 2"`]
2. With seed / hyperparameters: [...]
3. See error or unexpected behavior

**Expected Behavior**
A clear and concise description of what you expected to happen (for example, the expected loss or accuracy).

**Terminal Output**
Paste the full output, including any panic message or failing test:
```text
(paste output here)
```

**Additional Context**
Add any other context about the problem here: a failing gradient check, a math derivation, or a suspected cause.
