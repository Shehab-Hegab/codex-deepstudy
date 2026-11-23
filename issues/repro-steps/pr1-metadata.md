# PR #1 — https://github.com/Shehab-Hegab/codex/pull/1 (merged)

- Fork: `Shehab-Hegab/codex` (a fork of `openai/codex`; upstream is 2584 commits ahead; fork is stale, `main` at `44fe510`).
- Base: `main` = `44fe510ce3ee61c8ef623adcbf89b901c73ddd61` (fork's main).
- Head commit (squash): `965c570e2aa07dff9abcaa18b9b9576d68eee52b` (parent `44fe510`).
- Merged: `2026-11-23T21:34:15Z`.
- Diff: 4 files, +139/−2.
  - `codex-rs/shell-command/src/bash.rs` (+6/−2): `pub(crate)` → `pub` on `parse_shell_lc_literal_commands`.
  - `codex-rs/core/src/exec_policy.rs` (+15/−0): fallback to literal parser before raw-argv.
  - `codex-rs/core/src/exec_policy_tests.rs` (+51/−0): 2 regression tests (rows 4/6).
  - `codex-rs/shell-command/tests/issue_63384_repro.rs` (+67/−0): new repro tests.
- Patch file: `issues/repro-steps/pr1.patch` (fetch: `gh api repos/Shehab-Hegab/codex/pulls/1/files --jq '.[].patch'`).

## Why the fork and not upstream

`openai/codex` accepts external PRs in practice? — actually **no**: `openai/codex` does not accept external PRs (issue tracker/analysis contributions only). More importantly, the fork's `main` is 2584 commits behind upstream; GitHub's `merge-upstream` API needs the `workflow` OAuth scope (not available here). So the merged PR targets the fork. The fix is identical to the upstream-pinned patch (`63384.patch`, against `06d76ee`).
