# Tasks

## 1. Scaffolding

- [x] 1.1 Create the `dataset/` crate (edition 2024) with `serde` (derive) and `serde_json`, and add it to the workspace `members`; verify `cargo build -p dataset` succeeds
- [x] 1.2 Add a `commands-data` Makefile target (`cargo run --release -p dataset -- $(ARGS)`) and list it in `make help`; verify `make help` shows it

## 2. Building blocks (each with unit tests in the same file)

- [x] 2.1 `random.rs`: copy `Rng` and `shuffle` from `mnist`, add `choose(&[T]) -> &T` and `chance(p) -> bool`; verify same-seed tests and that `chance(0.0)`/`chance(1.0)` are never/always true
- [x] 2.2 `tokenize.rs`: the spec's tokenization rule; verify tests for both spec scenarios plus an empty string and repeated spaces
- [x] 2.3 `grammar.rs`: parse `(a|b)`, `[x]`, `{slot}` and nesting into a tree, and expand it to all phrasings; verify tests for the 24-phrasing example in design.md, nested groups, and clear errors for unbalanced brackets and empty options
- [x] 2.4 `template.rs`: parse `{slot}`, `{slot?}`, `{slot=default}` and fill a template from a slot map; verify tests for each form, a missing required slot error, and that the result never has double spaces

## 3. Catalog

- [x] 3.1 `catalog.rs`: the `Intent` struct, the slot pools (train and test lists per slot type), and the shared prefix/suffix wrappers
- [x] 3.2 Write the 50 command intents from design.md (decision 8), each with grammar lines and at least 3 hand-written test sentences
- [x] 3.3 Write the `unknown` intent: grammar for out-of-scope and unsupported or destructive requests, and at least 10 test sentences

## 4. Validation and generation

- [x] 4.1 `validate.rs`: every check from design.md (decision 9), each returning an error that names the intent; verify one unit test per check using small in-memory catalogs, and a test that the real catalog passes
- [x] 4.2 `generate.rs`: sample `per_intent` training examples per intent (deduplicated, with wrappers), build test examples from the test sentences and test pools, and produce tokens, BIO tags, slots and command; verify tests that tags align with tokens, that commands are reconstructible, and that the same seed gives the same examples
- [x] 4.3 Check train/test disjointness (lowercased text and slot values) on the generated examples; verify a test that a deliberately overlapping catalog is rejected

## 5. Command-line program

- [x] 5.1 `main.rs`: parse `--out --seed --per-intent --help`, validate, generate, write `train.jsonl`, `test.jsonl`, `intents.json` through temp files and rename, and print the summary; on error print the message and exit 1; verify `cargo run -p dataset -- --per-intent 0` exits non-zero with usage
- [x] 5.2 Add `dataset/README.md` (purpose, grammar syntax, slot forms, how to add an intent, file format); verify the documented commands run as written

## 6. Integration checks

- [x] 6.1 Run `make commands-data`; verify the three files exist, the summary shows ≥ 51 intents and ≥ 3,000 training examples, and `git status` shows nothing under `data/`
- [x] 6.2 Run the generator twice with the same seed into two directories; verify `cmp` reports the files identical
- [x] 6.3 Spot-check 20 random training lines and all test lines for natural English and correct labels; fix the catalog where needed
- [x] 6.4 Update `AGENTS.md` (workspace members), `.agents/shared/agent-startup.md` (module map) and `.agents/shared/memory/project-status.md`; run `make check` and `make spec` and verify both pass

## Workflow follow-up

- Archive with `openspec archive add-command-dataset` after review, which creates `openspec/specs/command-dataset`.
- Next change: `add-intent-classifier` (Burn: intent classifier + slot model trained on this dataset).
