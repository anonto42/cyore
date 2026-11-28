---
name: Feature Request
about: Suggest an idea or enhancement for Cyore
title: "[FEATURE] "
labels: enhancement
assignees: ''
---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem, learning goal, or use case is.

**Describe the Solution You'd Like**
A clear and concise description of the capability, model change, command, or tool you would like to see.

**Learning-Plan Stage**
Which stage of the [roadmap](https://github.com/anonto42/cyore_ai#learning-roadmap) does this belong to? [Stage 1 / 2 / 3 / 4]

**Describe Alternatives You've Considered**
A description of any alternative solutions, architectures, or workarounds you've considered.

**Invariants & Scope Confirmation**
- [ ] Stage 1 crates stay standard-library only (no ML, linear-algebra, or RNG crates).
- [ ] Any new math comes with a test or numerical gradient check.
- [ ] Any agent feature keeps confirm-before-run and the allowlist/blocklist ([Safety Rules](https://github.com/anonto42/cyore_ai/blob/main/.agents/shared/rules/safety.md)).
- [ ] Any new dependency is justified against the 5–30 MB binary / under 100 MB RAM target.

**Additional Context**
Add any other context, papers, videos, or examples about the feature request here.
