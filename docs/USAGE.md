# Usage

How to build and run every finished milestone. All commands run from the repository root.

---

## 1. Setup
```bash
git clone https://github.com/anonto42/cyore_ai.git
cd cyore_ai
nix develop          # or: direnv allow, or install rustup (it reads rust-toolchain.toml)
make help            # list every target
```

Most targets act on one crate, chosen with `CRATE` (default `xor`):
```bash
make run CRATE=xor
```

---

## 2. XOR (`xor/`)
```bash
make run             # optimised release build, then run
make dev             # debug build (compiles faster)
make size            # print the release binary size
```

The 2 → 4 → 1 network trains for 20,000 epochs with learning rate 1.0 and seed 42. It logs the average MSE every 2,000 epochs, then prints the truth table:

| Column | Meaning |
|---|---|
| `Input [A, B]` | The two inputs |
| `Target` | `A XOR B` |
| `Predicted` | The raw sigmoid output, between 0 and 1 |
| `Rounded` | The prediction rounded at 0.5 |

It reports success only if all four rounded predictions are correct. Otherwise it suggests another seed or more epochs. To experiment, change the seed, hidden size, epochs or learning rate in `xor/src/main.rs`.

---

## 3. MNIST (`mnist/`)
### Download the dataset (once)
```bash
make mnist-data      # needs curl and gunzip; writes data/mnist/ (git-ignored)
```
The four IDX files are downloaded only if missing, and an interrupted run never leaves a half-written file.

### Train and evaluate
```bash
make mnist                              # defaults: 10 epochs, lr 0.5, batch 10, 128 hidden, seed 42
make mnist ARGS="--epochs 2"            # quick run
make mnist ARGS="--hidden 64 --lr 0.3 --seed 7"
make mnist ARGS="--help"                # every option
```

| Option | Meaning | Default |
|---|---|---|
| `--epochs <N>` | Passes over the training set | `10` |
| `--lr <RATE>` | Learning rate | `0.5` |
| `--batch <N>` | Mini-batch size | `10` |
| `--hidden <N>` | Hidden-layer neurons | `128` |
| `--seed <N>` | Random seed (same seed = same run) | `42` |
| `--data <DIR>` | Directory with the four IDX files | `data/mnist` |

Each epoch prints the training loss and test accuracy. The run ends with a confusion matrix on the 10,000 test images. With the defaults, expect about **98% test accuracy**, about 6 s per epoch on a modern laptop, and a peak of about 430 MB of RAM (all pixels are held as `f64`).

---

## 4. Command dataset (`dataset/`)
```bash
make commands-data                          # writes data/commands/ (git-ignored)
make commands-data ARGS="--per-intent 120"  # more training examples per intent
make commands-data ARGS="--help"            # every option
```

| Option | Meaning | Default |
|---|---|---|
| `--out <DIR>` | Output directory | `data/commands` |
| `--seed <N>` | Random seed (same seed = byte-identical files) | `42` |
| `--per-intent <N>` | Training examples per intent | `80` |

With the defaults it writes 51 intents (50 commands + `unknown`), 4,080 training examples and 162 hand-written test examples. The format of `train.jsonl`, `test.jsonl` and `intents.json` is described in the [Dataset Crate Guide](../dataset/README.md).

---

## 5. Containers
```bash
make docker-run              # builds a static musl binary into an empty FROM scratch image
make docker-run CRATE=xor
```
The `mnist` binary needs the dataset at runtime, so run it directly with `make mnist` rather than in the container.

---

## 6. Prebuilt binaries
Tagged releases publish archives for Linux (x86_64 and aarch64, static musl), Windows (x86_64) and macOS (Apple Silicon). Download them from the [Releases page](https://github.com/anonto42/cyore_ai/releases) and check them with `sha256sum -c SHA256SUMS.txt`. See [RELEASES.md](RELEASES.md).
