# Git & GitHub Rules

These rules apply to every AI assistant (Claude, Codex, Gemini/Antigravity, Copilot, etc.) working in
this repository. They override any default attribution behavior built into an assistant or its harness.

1. **No AI Artifacts — Anywhere:**
   - Never add `Co-Authored-By:` trailers naming an AI model or vendor (e.g. Claude, Anthropic, OpenAI,
     Copilot, Gemini) to commit messages.
   - Never add "Generated with …", "🤖", "Written by AI", or similar footers/badges to commit messages,
     pull request titles or descriptions, issue text, review comments, or code comments.
   - Never mention the assistant, its model name, or the prompt in anything that lands in git history
     or on GitHub.

2. **Commit as the Real Git User:**
   - Use the repository's configured `user.name` / `user.email` (`git config user.name`). Never pass
     `--author`, set `GIT_AUTHOR_*` / `GIT_COMMITTER_*`, or edit git config to change identity.
   - Commits, PRs, and GitHub comments must read as written by the project owner: plain, first-person
     where natural, no AI disclaimers.

3. **Commit Message Style:**
   - Conventional Commits: `type(scope): summary` (e.g. `feat(xor): …`, `docs(openspec): …`).
   - Body explains *what* and *why*; nothing else after it.

4. **Before Committing or Posting:**
   - Re-read the full message/comment text and strip any AI attribution before running `git commit`,
     `gh pr create`, `gh pr comment`, `gh issue comment`, or similar.
   - Commit or push only when the owner asks. Never force-push `main` without explicit approval.
