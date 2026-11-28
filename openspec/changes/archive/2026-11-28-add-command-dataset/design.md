# Design

## Context

The workspace has two std-only Stage 1 crates (`xor`, `mnist`). The next change (Stage 2, Burn) will train an intent classifier (sentence → one of N intents) and a slot model (token → BIO tag). Both are supervised, so they need labelled examples whose labels are consistent by construction. See proposal.md for scope.

## Goals / Non-Goals

**Goals:**
- A catalog that a learner can read and extend in one place: one entry per intent, holding its command, its phrasings, and nothing else.
- Training data with enough surface variety that a small model must learn words, not memorise sentences.
- A test set that measures real generalisation: unseen sentence shapes *and* unseen slot values.

**Non-Goals:**
- Realistic frequency of requests (all intents are roughly balanced on purpose).
- Typos, spelling noise, or paraphrase models; hand-written grammar only.

## Decisions

1. **New `dataset/` crate with the catalog in Rust source.** Each intent is a struct literal in `catalog.rs`: name, command template, phrasing grammar lines, and hand-written test sentences. The compiler checks the structure and the generator validates the content.
   *Alternatives:* a TOML/YAML catalog (needs a parser crate, and errors surface later); a Python script (a second language in a Rust learning repo).

2. **Command templates use three slot forms:** `{dir}` required, `{dir?}` optional (the placeholder token disappears when no value is given), `{dir=.}` optional with a default. Example: `ls -la {dir?}` gives `ls -la` or `ls -la Downloads`, and `find {dir=.} -name {pattern}` gives `find . -name *.txt`.
   Commands are stored as plain strings whose whitespace-separated words are the program's arguments. Slot values contain no whitespace, so the Stage 3 agent can split a command into `std::process::Command` arguments without a shell. Without a shell, `*.txt` is passed to `find` literally, which is what `find -name` expects.

3. **Phrasing grammar.** One line describes many phrasings:
   - `(a|b|c)` picks exactly one option
   - `[x]` is optional (present or absent)
   - `{slot}` is a slot placeholder
   - groups can nest: `[in (the|this) folder]`

   `(show|list|display) [me] [all] the files [in {dir}]` expands to 3 × 2 × 2 × 2 = 24 phrasings. A small recursive-descent parser turns a line into a tree and expands it to every combination. Writing the grammar by hand teaches what variety the model needs; the expansion is the "generate variations with scripts" step of the plan.
   *Alternative:* listing every sentence by hand. That is 1,000+ lines and hard to keep consistent.

4. **Sampling to a fixed size per intent.** Expansions vary from 20 to thousands per intent. For each intent the generator draws `per_intent` examples (default 80): pick a random expansion, fill each slot from the training pool, and apply the shared wrappers (below). Duplicate texts are re-drawn up to a limit. Equal counts keep the classifier from favouring intents with bigger grammars. 51 intents × 80 gives about 4,000 training examples.

5. **Wrappers for natural variety, chosen by sentence kind.** A prefix like "can you" fits an instruction ("can you show the files") but not a question ("can you what is my ip") or a bare phrase ("can you disk space"). So each grammar line starts with `do:` (instruction), `ask:` (question), or nothing (bare phrase). At sampling time, instructions may get a prefix (`please`, `can you`, `could you`, `would you`, `go ahead and`, `hey,`), a suffix (`please`, `for me`, `now`), and a `?` after "… you" or else an occasional `.`. Questions may get `hey,`, `quick question,`, `so`, `ok,` and a `?`. Bare phrases may get a `?`. Any sentence may be capitalised. Wrappers are not counted as distinct phrasings, so the ≥ 20 rule still measures real variety.

6. **Slot pools, split train/test.** Each slot type has two disjoint value lists, e.g. `dir` train: `Downloads, Documents, src, ~/projects, /var/log, …`; `dir` test: `Pictures, build, ~/notes, /etc, …`. Test sentences are written with `{slot}` placeholders and filled from the test pools, so the slot model is graded on values it has never seen. It has to learn from context ("in X", "called X") rather than memorise names. Slot types: `dir`, `file`, `pattern`, `text`, `host`, `process`, `program`.

7. **The `unknown` intent** has its own grammar of out-of-scope requests (`write me a poem`, `what is the weather`) and of requests the dataset deliberately does not support (`delete all my files`, `format the disk`, `shut down the computer`). Training on these gives the classifier a place to put such requests, so the Stage 3 agent can ask the user instead of guessing a nearby command. It has no slots and a null command.

8. **Intent list (50 command intents):**
   - Files (read): `list_files` `ls -la {dir?}`, `list_by_size` `ls -lhS {dir?}`, `list_by_time` `ls -lt {dir?}`, `current_dir` `pwd`, `show_file` `cat {file}`, `show_file_numbered` `cat -n {file}`, `show_head` `head -n 20 {file}`, `show_tail` `tail -n 20 {file}`, `count_lines` `wc -l {file}`, `file_type` `file {file}`, `file_info` `stat {file}`, `file_checksum` `sha256sum {file}`, `find_by_name` `find {dir=.} -name {pattern}`, `search_text` `grep -rn {text} {dir=.}`, `dir_size` `du -sh {dir=.}`
   - Files (create, non-destructive): `make_dir` `mkdir -p {dir}`, `create_file` `touch {file}`
   - Disks and memory: `disk_space` `df -h`, `disk_inodes` `df -i`, `memory` `free -h`, `block_devices` `lsblk`, `mounted_filesystems` `findmnt`
   - System: `os_info` `uname -a`, `kernel_version` `uname -r`, `hostname` `hostname`, `uptime` `uptime`, `whoami` `whoami`, `date` `date`, `calendar` `cal`, `cpu_info` `lscpu`, `cpu_count` `nproc`, `processes` `ps aux`, `top_cpu` `ps aux --sort=-%cpu`, `top_memory` `ps aux --sort=-%mem`, `find_process` `pgrep -a {process}`, `logged_in_users` `who`, `env_vars` `env`, `show_path` `printenv PATH`, `which_program` `which {program}`
   - Network: `ip_addresses` `ip addr`, `routes` `ip route`, `ping_host` `ping -c 4 {host}`, `open_ports` `ss -tuln`, `dns_lookup` `getent hosts {host}`
   - Git: `git_status` `git status`, `git_log` `git log --oneline -n 10`, `git_diff` `git diff`, `git_branches` `git branch`, `git_current_branch` `git branch --show-current`, `git_remotes` `git remote -v`

   The allowlist is exactly the set of programs above. Some intents differ only in nuance (`list_by_size` vs `dir_size`, `processes` vs `top_cpu`). That is deliberate: these are the hard cases that show what a small model can and cannot tell apart.

9. **Validation before writing.** The generator checks everything first, then writes each file to a temporary name and renames it, so a failure never leaves partial output. Checks: unique intent names; grammar parses; template slots and phrasing placeholders agree (required slots appear in every expansion, no unknown placeholders); ≥ 20 distinct expansions per intent; ≥ 3 test sentences per command intent and ≥ 10 for `unknown`; allowlist and blocklist; no shell metacharacters; train/test text and slot-value disjointness.

10. **`serde` + `serde_json` for output.** Hand-escaping JSON strings is easy to get subtly wrong (control characters, Unicode). The Burn crate in the next change will read these files with serde anyway, so both sides share one well-tested implementation. The dependency exists only in the generator binary, not in anything shipped, so it does not affect the Stage 4 footprint target. `Cargo.lock` pins the versions.

11. **CLI:** `dataset [--out DIR] [--seed N] [--per-intent N]`, defaults `data/commands`, 42, 80, hand-parsed like `mnist`. `make commands-data` runs it in release mode.

## Risks / Trade-offs

- [Grammar-generated sentences are more regular than real users] → the hand-written test set is the honest measure; the model change reports accuracy on it separately from training accuracy.
- [Near-synonym intents may cap accuracy] → accepted on purpose (decision 8); the confusion matrix in the next change will show which ones.
- [Typing 50 grammars and 150+ test sentences is real work] → the minimum per intent is small (20 expansions is often a single grammar line); the catalog can grow over time without code changes.
- [Commands differ slightly across distributions (`ss`, `ip`, `findmnt` are Linux-specific)] → the target is Linux first, as in the README; portability is a later concern.

## Migration Plan

Additive only: new crate, one Makefile target. Rollback is removing `dataset` from the workspace members.
