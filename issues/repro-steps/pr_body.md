## Summary

Fixes a real `execpolicy` bypass in `openai/codex`: a forbidden `prefix_rule` is silently skipped whenever a `zsh -lc` script contains `~` or `$HOME` anywhere, because the strict parse fails and policy checks the opaque `["/bin/zsh", "-lc", script]` argv instead of the literal commands inside.

## Root cause

`commands_for_exec_policy_for_platform()` (`codex-rs/core/src/exec_policy.rs`) calls `parse_shell_lc_plain_commands()`; when that returns `None`, it currently falls back to the raw argv, so no prefix rule for `git`/`cat` can match. The parser rejects a *whole* script when any word contains `~`/`$` (they can expand), even when the forbidden command is spelled fully literally in the same script.

## Fix

Fall back to the existing `parse_shell_lc_literal_commands()` (`codex-rs/shell-command/src/bash.rs`) before the raw-argv fallback, so literal subcommands in the script are still checked against prefix rules. Made that parser `pub` (it was `pub(crate)`).

## Testing

- 2 regression tests in `exec_policy_tests.rs` (rows 4 & 6 of the issue): literal `cat`/`git` must be exposed and the result must not be the raw argv.
- A 3-test repro in `shell-command/tests/issue_63384_repro.rs` pins the trigger (strict parse is `None`) plus a control.
- Verified against the real `tree-sitter-bash` grammar with a 1:1 port of the parser logic: strict parse returns `None` for the issue rows, literal fallback recovers them; adversarial cases (substitutions, quoted `$HOME`, redirects, string args, comment-only) pass.

Closes the bypass class of https://github.com/openai/codex/issues/63384. Rows where `~`/`HOME` is the path itself still need tilde/env normalization — out of scope.
