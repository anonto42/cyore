# Operational Safety & Security Rules

These rules matter most from Stage 3 onward, when the model's output is executed as a shell command.

1. **Never Execute Unconfirmed Commands:**
   - The agent shows the generated command and asks `Run this? (y/n)` before every execution.
   - Execute via `std::process::Command` with explicit arguments; never pass model output through
     `sh -c` unless the command template was chosen from the allowlist.

2. **Allowlist and Blocklist:**
   - Only commands whose program is on the allowlist may run.
   - Always refuse destructive patterns regardless of confidence: `rm -rf /`, `mkfs`, `dd`,
     `shutdown`, `reboot`, fork bombs, writes to `/dev/sd*`, and `chmod -R 777 /`.

3. **Confidence Threshold:**
   - If the model's confidence is below the threshold, ask the user to clarify instead of guessing.

4. **Timeouts and Graceful Failure:**
   - Every executed command has a timeout. Failures return an informative message; the agent never
     panics on bad model output or a failed command.

5. **No Secrets in the Repo:**
   - Never commit API keys, tokens, or credentials. Training datasets must not contain real secrets,
     hostnames, or personal paths.
