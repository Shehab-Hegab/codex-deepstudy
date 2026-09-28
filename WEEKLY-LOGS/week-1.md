# Week 1 Log — Deep Study Begins

**Date**: September 28, 2026  
**Focus**: Source code mapping, environment setup, architecture discovery

## ✅ Completed

### Environment Setup
- [x] Clone `openai/codex` at `C:\OpenAI\codex` (shallow depth 1, commit `21eb355`)
- [x] Install Rust 1.98.1 via rustup at `C:\Users\Shehab\.cargo\bin`
- [x] Install `just` 1.58.0 and `ripgrep` 15.1.0 to `C:\Users\Shehab\.cargo\bin`
- [x] Fix PATH for cargo/rustc/just/rg
- [x] Verify `just --list` in `codex-rs/` — Bazel-based build system confirmed
- [x] Create `codex-deepstudy` repo at https://github.com/Shehab-Hegab/codex-deepstudy
- [x] Create `openai-codex-lab` workspace with full directory structure

### Source Code Mapping
- [x] Read `codex-core/src/lib.rs` (252 lines) — all public APIs and module structure
- [x] Read `codex-core/src/agent/` — AgentControl trait, LocalAgentControl, execution limiter, registry, role, types, spawn logic
- [x] Read `codex-core/src/exec/exec.rs` (1276 lines) — ExecParams, process_exec_tool_call(), sandbox integration
- [x] Read `codex-core/src/exec/spawn.rs` (137 lines) — spawn_child_async(), sandbox process management
- [x] Read `codex-core/src/agent/control/spawn.rs` (1433 lines) — SpawnInitialInput, AGENT_NAMES, restore_v2_agent_metadata()
- [x] Read `codex-core/src/session/turn_input.rs` (780 lines) — TurnStartKind, PreparedTurnInputSettings
- [x] Read `codex-core/src/session/turn_context.rs` (1392 lines) — TurnEnvironment, ShellSnapshotTask, ShellSnapshotCache
- [x] Read `codex-core/src/session/mod.rs` (5211 lines) — central session module
- [x] Read `codex-core/src/guardian/mod.rs` (210 lines) — GuardianReviewContext, review sessions, approval flow
- [x] Read `codex-protocol/src/turn_input.rs` (252 lines) — TurnInput enum, TurnInputRequest, submission types
- [x] Read `codex-protocol/src/items.rs` (874 lines) — TurnItem enum (18 variants)
- [x] Read `codex-protocol/src/approvals.rs` (548 lines) — GuardianRiskLevel, permission profiles
- [x] Read `codex-protocol/src/protocol.rs` (1404 lines) — AgentStatus, CollaborationMode, etc.
- [x] Map 130+ crates in workspace

### Documentation Created
- [x] `architecture-notes/01-agent-execution-loop.md` — Complete architecture mapping
- [x] `PLAN.md` — Original execution plan
- [x] `90-DAY-PLAN.md` — Compressed 90-day plan for OpenAI candidacy
- [x] `WEEKLY-LOGS/week-1.md` — This file

## 📊 Key Discoveries

1. **Agent Control Flow**: `TurnInput` → `turn_input processing` → `TurnContext` → `exec/spawn` → `sandbox` → results
2. **Guardian System**: Sits between agent and execution — permission checking, approval requests, review decisions
3. **Agent Execution Limiter**: Only V2 subagents limited via AtomicUsize + OnceLock<usize>
4. **130+ crates**: Bazel build system, core/protocol/sandboxing/exec-server are the main entry points
5. **No external PRs**: OpenAI does NOT accept external contributions — strategy must shift to issues/analysis/tools

## 🔜 Week 2 Goals

- [ ] Read `thread_manager.rs`, `rollout/`, `codex_thread.rs`
- [ ] Read `exec-server/src/` and `sandboxing/src/`
- [ ] Begin Rust Book chapters 1-5 (ownership, borrowing)
- [ ] Start 40 Rust exercises
- [ ] Browse openai/codex issues for first filings

## 🎯 Monthly Target

- 2 high-quality issues filed on openai/codex
- 1 architecture note published
- Rust fundamentals: chapters 1-5

---

*Daily commits, weekly logs, never skip.*
