# OpenSpec Changes Directory

This directory stores active and proposed changes following the Spec-Driven Development (SDD) process:

- Propose a change: `openspec-propose` skill, or create `<change-name>/proposal.md`
- Design the change: add `design.md`, `tasks.md`, and delta specs under `specs/`
- Implement & verify: run `make check`
- Archive: `openspec archive <change-name>` moves it to `archive/` and syncs main specs
