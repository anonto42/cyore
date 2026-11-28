# Releases and Packaging

How Cyore is versioned, built for each platform, and published.

---

## 1. Versioning

Cyore follows **Semantic Versioning** (`vMAJOR.MINOR.PATCH`). Before `v1.0.0`, minor versions track the learning plan:

- `MINOR`: A new milestone crate or capability (for example, MNIST, the intent classifier, the agent).
- `PATCH`: Bug fixes, better tests, and documentation improvements.
- `MAJOR` (`v1.0.0`): The packaged Stage 4 agent: one binary with embedded int8 weights.

### Creating a Release Tag
```bash
# Make sure the working tree is clean and every check passes
make check
make spec

# Create an annotated git tag
git tag -a v0.2.0 -m "v0.2.0: MNIST digit classifier in pure Rust"

# Push the tag to GitHub
git push origin v0.2.0
```

Pushing a tag that matches `v*.*.*` starts `.github/workflows/release.yml`.

---

## 2. What the Release Workflow Builds

Each target runs on its own native GitHub runner. The workflow runs the full test suite, builds every binary listed in `BINS` with the shared release profile, and packages it together with `README.md` and `LICENSE`:

| Target | Runner | Archive |
|---|---|---|
| `x86_64-unknown-linux-musl` | `ubuntu-latest` | `cyore-<tag>-x86_64-unknown-linux-musl.tar.gz` (static) |
| `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` | `cyore-<tag>-aarch64-unknown-linux-musl.tar.gz` (static; Raspberry Pi 3/4/5 on a 64-bit OS) |
| `x86_64-pc-windows-msvc` | `windows-latest` | `cyore-<tag>-x86_64-pc-windows-msvc.zip` |
| `aarch64-apple-darwin` | `macos-latest` | `cyore-<tag>-aarch64-apple-darwin.tar.gz` |

A final job collects every archive, writes `SHA256SUMS.txt`, and publishes a GitHub Release with auto-generated release notes.

To ship a new milestone binary, add its crate name to `BINS` at the top of the workflow.

---

## 3. Container Image

The `Dockerfile` builds a fully static musl binary for one crate and copies it into an empty `FROM scratch` image that runs as an unprivileged user:

```bash
make docker-build CRATE=xor
make docker-run CRATE=xor
```

---

## 4. Footprint Budget

Stage 4 targets a **5–30 MB** binary that uses **under 100 MB of RAM**. Check the size of any release build with:

```bash
make size CRATE=xor
```

The release profile in the root `Cargo.toml` already uses `lto = true`, `codegen-units = 1`, `strip = true` and `panic = "abort"`. Switch `opt-level` to `"z"` when size matters more than speed.

---

## 5. Verifying Downloads

Every release includes `SHA256SUMS.txt`. Verify an archive before you use it:

```bash
sha256sum -c SHA256SUMS.txt --ignore-missing
```
