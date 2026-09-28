# 01 — Agent Execution Loop

## Overview

OpenAI Codex is an AI agent system where the core execution loop is: User → Agent Harness → Model → Tool Call → Execution (sandbox) → Observation → Model → Next Action → ...

The system is built in Rust across 130+ crates, using Bazel as the build system (orchestrated via `just`).

## Top-Level Crate Architecture

### `codex-core` (`codex-rs/core/src/lib.rs`, 252 lines)
The central crate — exports all public APIs and modules:

**Public Re-exports (from codex-protocol):**
- `TurnInput`, `TurnInputRequest`, `TurnInputSubmission`, `TurnStartOptions`, `RecoverTurnRequest`
- `NotSubmittedReason`, `StartIfIdleSubmission`, `SteerSubmission`, `SuspendTurnOutcome`

**Agent API:**
- `AgentControl` — trait defining `identity()`, `resolve()`, `spawn()`, `send()`, `ensure_child_loaded()`, `interrupt()`
- `AgentConfigUpdate`, `AgentInfo`, `AgentInput`, `AgentTarget`, `AgentTurnOutcome`, `DeliveryReceipt`, `SendRequest`, `SpawnRequest`

**Agent Types:**
- `AgentExecutionGuard` — permit-based execution constraint (Drop impl decrements counter)
- `AgentMessage` — `Plaintext(String)` | `Encrypted(String)`
- `AgentMetadata`, `LiveAgent`, `MessageDeliveryMode` (`QueueOnly` | `TriggerTurn`)
- `SpawnAgentForkMode` (`FullHistory` | `LastNTurns(usize)`)
- `SpawnAgentOptions` — fork_parent_spawn_call_id, fork_mode, parent_thread_id, parent_turn_id, turn_trigger, root_turn_id, environments, multi_agent_v2_usage_hints, cyber_access_program

**Thread System:**
- `CodexThread` — conversation thread
- `CodexThreadSettingsOverrides`, `ThreadConfigSnapshot`, `BackgroundTerminalInfo`, `ThreadStartupMetadata`
- `GuardianRootMessage`, `GuardianRootSnapshot`

**Session:**
- `TurnContext` — context for a single turn

**Exec:**
- `pub mod exec` (1276 lines) — `ExecParams`, `ExecExpiration`, `process_exec_tool_call()`, `build_exec_request()`
- `pub mod exec_env` — execution environment
- `mod exec_policy` — execution policies

**Guardian:**
- `mod guardian` (210 lines) — `GuardianReviewContext`, `GuardianReviewSession`, `GuardianReviewSessionManager`, `GuardianReviewState`
- `pub mod guardian_review` — review system

**Other Modules:**
- `config`, `context`, `compact`, `realtime_conversation`, `agent_communication`, `agent_message_board`, `rollout`, `rollout_budget`, `client`, `sandboxing`, `windows_sandbox`, `tools`, `shell`, `shell_snapshot`, `network_policy_decision`, `codex_network_proxy`, `skills`, `plugins`, `mention_syntax`, `util`, `test_support`, `state_db_bridge`

### `codex-protocol` (`codex-rs/protocol/src/lib.rs`, 56 modules)
Defines the protocol/communication types used across the system:

**Key modules:**
- `turn_input.rs` (252 lines) — `TurnInput` enum (`UserInput`, `ResponseItem`, `InterAgentCommunication`), `TurnInputRequest`, `TurnInputSubmission`, `TurnStartOptions`, `RecoverTurnRequest`
- `user_input.rs` — `UserInput` enum (`Text`, `Image`, `LocalImage`, `Audio`, `LocalAudio`, `Skill`, `Mention`), `TextElement`, `ByteRange`
- `items.rs` (874 lines) — `TurnItem` enum with many variants: `UserMessage`, `FunctionCallOutput`, `HookPrompt`, `AgentMessage`, `Plan`, `Reasoning`, `CommandExecution`, `DynamicToolCall`, `CollabAgentToolCall`, `SubAgentActivity`, `WebSearch`, `ImageView`, `Extension`, `ImageGeneration`, `EnteredReviewMode`, `ExitedReviewMode`, `FileChange`, `McpToolCall`, `ContextCompaction`
- `approvals.rs` (548 lines) — `ResolvedPermissionProfile`, `EscalationPermissions`, `ExecPolicyAmendment`, `NetworkApprovalProtocol`, `NetworkApprovalContext`, `NetworkPolicyRuleAction`, `GuardianRiskLevel` (`Low`/`Medium`/`High`/`Critical`), `GuardianUserAuthorization`, `GuardianAssessmentAction/Event/Outcome/Status`, `GuardianCommandSource`
- `protocol.rs` (1404 lines) — Central definitions: `AgentStatus`, `CollaborationMode`, `ModeKind`, `MultiAgentMode`, `Personality`, etc.

### Other Major Crates
- **`codex-rs/exec/`** — Exec server implementation
- **`codex-rs/sandboxing/`** — Sandbox implementations (Linux landlock, Windows restricted token, managed network)
- **`codex-rs/exec-server/`** — Execution server that runs sandboxed commands
- **`codex-rs/guardian-context/`**, **`codex-guardian-reviewer/`** — Guardian approval and review
- **`codex-rs/agent-graph-store/`**, **`agent-identity/`**, **`agent-message-board-client/`**, **`agent-roles/`** — Agent state, identity, communication, roles
- **`codex-rs/thread-store/`**, **`state/`** — Persistence
- **`codex-rs/rollout/`** — Rollout recording
- **`codex-rs/model-provider/`**, **`models-manager/`** — Model integration
- **`codex-rs/app-server/`** — Application server
- **`codex-rs/shell-command/`**, **`shell-escalation/`** — Shell execution
- **`codex-rs/network-proxy/`**, **`codex_network_proxy/`** — Network proxy
- **`codex-rs/windows-sandbox-rs/`**, **`windows-sandbox-service/`** — Windows sandboxing
- **`codex-rs/skills/`**, **`plugins/`** — Skills and plugin system
- **`codex-rs/tui/`** — Terminal UI
- **`codex-rs/codex-api/`**, **`codex-client/`**, **`codex-mcp/`** — API client, MCP integration

## Agent Execution Flow (Detailed)

```
TurnInput (from protocol)
    ↓
core/session/turn_input.rs — processes TurnInput (780 lines)
    ↓
core/session/turn_context.rs — creates TurnContext (1392 lines)
    TurnEnvironment: selection, config_origin, environment, shell, executor_platform_os
    ShellSnapshotTask, ShellSnapshotCache
    ↓
core/agent/control/ — agent control logic
    ├── control.rs — LocalAgentControl: send_input(), send_inter_agent_communication()
    │   Uses ThreadManagerState (Weak reference) for thread access
    ├── execution.rs — AgentExecutionLimiter (AtomicUsize + OnceLock<usize>)
    │   AgentExecutionGuard (permit-based), LocalExecutionPermit (Drop impl)
    │   Only V2 subagents are limited
    ├── spawn.rs — SpawnInitialInput, AGENT_NAMES, restore_v2_agent_metadata()
    │   prepare_agent_spawn_config(), keep_forked_rollout_item()
    │   1433 lines - the largest file
    ├── delivery.rs — AgentMessage::into_communication()
    ├── completion.rs — notify_parent_of_terminal_turn()
    └── api.rs — AgentControl trait definition
    ↓
core/agent/registry.rs — AgentRegistry (Mutex<ActiveAgents> + AtomicUsize)
    ActiveAgents: agent_tree, thread_paths, used_nicknames
    AgentMetadata: agent_id, agent_path, agent_nickname, agent_role
    ↓
core/agent/role.rs — AgentRoleOverrides (developer_instructions, model, reasoning_effort, etc.)
    DEFAULT_ROLE_NAME = "default"
    ↓
core/agent/types.rs — LiveAgent, AgentStatus (PendingInit|Running|Completed|Errored|Interrupted|Shutdown)
    ↓
core/agent/child_config.rs — SpawnConfigVersion (V1|V2), model_supports_multi_agent_backend()
    ↓
core/exec/spawn.rs — spawn_child_async() — spawns child process with sandboxing
    StdioPolicy (RedirectForShellTool | Inherit)
    Platform-specific process group management
    ↓
core/exec/exec.rs — process_exec_tool_call() — main entry for tool execution
    Uses codex_sandboxing::SandboxManager, SandboxCommand, SandboxType
    ExecParams: command, cwd, expiration, capture_policy, env, network, sandbox_permissions, windows_sandbox_level
    tokio::process::Child for async process management
    ↓
Sandbox execution (Linux landlock / Windows restricted token / managed network)
    ↓
Results returned as TurnItem → appended to conversation → Model receives observation
    ↓
Loop continues until: AgentTurnOutcome (Completed|Errored|Interrupted|Shutdown)
```

## Guardian System

Sits between agent and execution — checks permissions, requests approval, reviews decisions:

- `GuardianReviewContext`: parent_response_id, turn, environments, model_info, reasoning_effort, approval_policy, approvals_reviewer
- `GuardianReviewSession`: Tracks review state
- `GuardianReviewSessionManager`: Manages multiple reviews
- Constants: `GUARDIAN_REVIEWER_NAME`, `GUARDIAN_MAX_ROOT_MESSAGE_TOKENS` (900), `GUARDIAN_MAX_NODE_REPL_TOOL_RESULT_TOKENS` (6000)
- `GuardianRiskLevel`: Low/Medium/High/Critical
- `GuardianUserAuthorization`, `GuardianAssessmentAction/Event/Outcome/Status`
- Modules: approval_request, coverage, decision, feedback, input_budget, permissions, prompt, request_budget, review, review_session, reviewer_config, runtime

## Agent Execution Limiter

```rust
AgentExecutionLimiter: AtomicUsize counter + OnceLock<usize> for max_threads
AgentExecutionGuard: permit-based guard tied to limiter
LocalExecutionPermit: Drop impl decrements counter
is_execution_limited(): Only V2 subagents are limited
```

## Agent Registry

```rust
AgentRegistry: Mutex<ActiveAgents> + AtomicUsize total_count
ActiveAgents:
  - HashMap<String, AgentMetadata> (agent_tree)
  - HashMap<ThreadId, RegisteredAgent> (thread_paths)
  - HashSet<String> (used_nicknames)
AgentMetadata: agent_id (Option<ThreadId>), agent_path (Option<AgentPath>), agent_nickname (Option<String>), agent_role (Option<String>)
```

## ThreadManager

```rust
ThreadManager — manages thread lifecycle
ThreadManagerState — Weak reference to thread state
NewThread — represents a new thread creation
ForkSnapshot — snapshot for forking threads
```

## Key Numbers

| File | Lines |
|------|-------|
| `core/src/lib.rs` | 252 |
| `core/src/session/mod.rs` | 5211 |
| `core/src/exec/exec.rs` | 1276 |
| `core/src/session/turn_context.rs` | 1392 |
| `core/src/session/turn_input.rs` | 780 |
| `protocol/src/turn_input.rs` | 252 |
| `protocol/src/protocol.rs` | 1404 |
| `protocol/src/items.rs` | 874 |
| `protocol/src/approvals.rs` | 548 |
| `core/src/agent/control/spawn.rs` | 1433 |
| `core/src/exec/spawn.rs` | 137 |
| `core/src/guardian/mod.rs` | 210 |

## Open Questions (for continued study)

1. How does `TurnContext` manage state across turns?
2. What is the exact flow from `TurnInput` → `TurnContext` → `exec`?
3. How does `ThreadManager` handle thread lifecycle (fork, snapshot, restore)?
4. How does `GuardianReviewSession` interact with `exec` for permission checking?
5. How does `RolloutRecorder` capture agent execution?
6. What is the `agent_message_board` protocol for inter-agent communication?
7. How does `realtime_conversation` differ from regular conversation flow?
8. How does `compact` handle context compaction?
9. What is the `codex_network_proxy` architecture?
10. How do `skills` and `plugins` integrate into the agent runtime?

## Notes

This note documents the complete architecture of OpenAI Codex as traced from source code.
Build system: Bazel, orchestrated via `just` commands in `codex-rs/`.
Rust 1.98.1 via rustup at `C:\Users\Shehab\.cargo\bin`.
