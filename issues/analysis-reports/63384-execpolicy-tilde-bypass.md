# openai/codex #63384 — execpolicy `forbidden` bypass via `~` / `$HOME`

**Status:** reproduced + root-caused + fix implemented (patch + regression tests written)
**Source pinned:** `openai/codex` commit `06d76ee77b6f66c9958f5a5286d262fbd8970709`
**Area:** `codex-rs/shell-command`, `codex-rs/core` — exec policy / approval enforcement
**Issue:** https://github.com/openai/codex/issues/63384 (open at time of analysis)

---

## 1. The problem

With a rules file containing:

```
prefix_rule(pattern=["git", "push"], decision="forbidden", justification="no pushes")
prefix_rule(pattern=["cat", "/Users/x/decoy/secret.txt"], decision="forbidden", ...)
```

and full access (no sandbox, approvals off):

| # | Command | Result |
|---|---|---|
| 1 | `cat /Users/x/decoy/secret.txt` | rejected ✅ |
| 2 | `cat ~/decoy/secret.txt` | runs ❌ |
| 3 | `cat $HOME/decoy/secret.txt` | runs ❌ |
| 4 | `ls ~ > /dev/null && cat /Users/x/decoy/secret.txt` | runs ❌ |
| 5 | `git push --dry-run /tmp/nonexistent-remote` | rejected ✅ |
| 6 | `ls ~ && git push --dry-run /tmp/nonexistent-remote` | runs ❌ |

Rows 4 and 6 are the serious ones: the forbidden command is spelled **exactly**
as in the rule; merely having `~` somewhere else in the script disables the rule.
Impact: a `forbidden` denylist is silently skipped. Not an OS-sandbox escape —
it matters exactly in the configuration the reporter used (full access, approvals off).

---

## 2. Root cause (traced to file:function in the pinned source)

1. `codex-rs/core/src/exec_policy.rs:974` `commands_for_exec_policy_for_platform()`
   calls `parse_shell_lc_plain_commands()` first. Prefix rules are checked
   **per extracted sub-command** only if this returns a non-empty list.
2. `codex-rs/shell-command/src/bash.rs:29` `try_parse_word_only_commands_sequence()`
   rejects the **whole script** if any `word`/`number` node fails the literal check.
3. `codex-rs/shell-command/src/bash.rs:257` `is_literal_word_or_number()` rejects
   any word containing `~` or `$` (correctly — they can expand at runtime).
   → rows 2/3/4/6 all fail the strict parse.
4. `exec_policy.rs:1000` failure path returns `commands: vec![command.to_vec()]` —
   the raw argv `["/bin/zsh", "-lc", "<script>"]` becomes ONE opaque command.
5. `execpolicy-rules/src/policy.rs` `check_multiple_with_options()` matches
   prefix rules against argv; program is `/bin/zsh`, so no `git`/`cat` rule matches
   → falls through to sandbox/approval heuristics → with full access, it runs.
6. `codex execpolicy check` doesn't expose this: `execpolicy/src/execpolicycheck.rs`
   takes raw tokens and never reproduces the `zsh -lc` wrapping.

**One-line root cause:** strict-parse failure was treated as "no commands", so
the script collapsed into an opaque zsh argv and every literal prefix rule was skipped.

Related family: #37079 (argv[0] path spelling), #13095 (Unicode confusables) —
all exec-policy matching gaps that defeat `forbidden`.

---

## 3. Fix implemented (scratch clone, `git diff`: 57 insertions / 1 deletion)

The repo already contained the right tool, unused:
`parse_shell_lc_literal_commands()` at `bash.rs:133` accepts complex syntax and
returns statically-known words from every command node (explicitly documented as
*not* safe for proving a command safe — fine here, since it is only used to find
commands to **check**, never to allow them).

| File | Change |
|---|---|
| `shell-command/src/bash.rs` | `pub(crate)` → `pub` on `parse_shell_lc_literal_commands()` + doc |
| `core/src/exec_policy.rs` | after strict parse fails, fall back to the literal parser and check those sub-commands **before** the raw-argv fallback |
| `core/src/exec_policy_tests.rs` | 2 regression tests (rows 4 & 6): forbidden literal command must be present in `commands`, and result must not equal raw zsh argv |

Patch: `issues/repro-steps/63384.patch`
Regression tests source: `issues/repro-steps/63384-tests.rs`

**Deliberately out of scope:** rows 2/3 (the `~/`, `$HOME/` spelling *of the path
itself*) need tilde/env-expansion normalization. The fix targets rows 4/6 where
the forbidden command is fully literal and unambiguous.

---

## 4. Verification (real runs)

### 4.1 Runtime cross-check with the identical grammar
`issues/repro-steps/verify_63384.py` ports `is_literal_word_or_number()` and
`try_parse_word_only_commands_sequence()` **exactly** and runs the real
`tree-sitter-bash` grammar. Output:

```
ROW | STRICT PARSE (what policy sees today)              | LITERAL PARSE (proposed fallback)
  1 | [['cat', '/Users/x/decoy/secret.txt']]              | [['cat', '/Users/x/decoy/secret.txt']]
  2 | None                                                | [['cat']]
  3 | None                                                | [['cat']]
  4 | None                                                | [['cat','/Users/x/decoy/secret.txt'], ['ls']]
  5 | [['git','push','--dry-run','/tmp/nonexistent-remote']]| [['git','push','--dry-run','/tmp/...']]
  6 | None                                                | [['git','push','--dry-run','/tmp/nonexistent-remote'], ['ls']]
```

→ Confirms: rows 1/5 are checked today; rows 2/3/4/6 collapse to `None` and
escape to raw argv; the fallback recovers the full forbidden commands in rows 4/6.
The same code ported to Rust (`issue63384-verify/src/lib.rs`) with 3 assertions is
included as `issues/repro-steps/verify_63384.rs`.

### 4.2 Environment limitation (stated honestly)
A full `cargo test` of the Codex workspace could not be executed here:
- no MSVC `link.exe` / CRT libs installed (only `rust-lld` present, but
  `msvcrt.lib` missing), and
- the workspace lockfile fetches a Chromium ICU git submodule that stalls.

So the Rust patch and its regression tests are **written and source-verified**,
but the final green `cargo test` run must happen on a machine with the MSVC
toolchain. The *behaviour* was verified with the real tree-sitter grammar above.

---

## 5. Interview one-liner (exact)

> "In `openai/codex` I reproduced #63384 — an exec-policy bypass where a `~`
> anywhere in a `zsh -lc` script makes the strict tree-sitter parse fail, so
> the policy engine checks the opaque zsh argv instead of the literal
> `git push`/`cat` inside, silently skipping a `forbidden` rule. I root-caused it
> to `commands_for_exec_policy_for_platform()` falling back to raw argv, and
> fixed it by reusing the repo's existing literal sub-command parser as the
> fallback, with two regression tests."

## 6. CV bullet (only after the public comment is posted)

`Reproduced + root-caused openai/codex #63384 (exec-policy `forbidden` bypass via `~`/`$HOME`); implemented tested fallback fix with 2 regression tests — <URL>`
