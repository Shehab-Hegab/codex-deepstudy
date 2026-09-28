# OpenAI Codex Apprenticeship — Complete Execution Plan

> **Target**: Deep study of OpenAI Codex (agent runtime) as preparation for Agent Systems roles
> **User**: Shehab-Hegab — AI Agent Architect, 7275 GitHub followers
> **Start Date**: September 28, 2026
> **Estimated Duration**: 12–16 weeks

---

## Current State Analysis

| Item | Status |
|------|--------|
| **GitHub Profile** | Shehab-Hegab, AI/ML Expert @ Niibu Inc, Agent Architect |
| **Skills** | GenAI, RAG, CV, Llama-3, LangGraph, Production Agentic Workflows |
| **Workspace** | `C:\OpenAI\` — empty |
| **Codex Repo** | Cloned to `C:\OpenAI\codex\` (shallow, depth 1) |
| **Rust** | NOT installed — required |
| **Node.js** | v24.15.0 — available |
| **Repo Structure** | 130 crates in `codex-rs/`, Bazel build system |
| **Contribution Policy** | NO external PRs accepted — issues/bug reports/analysis only |

---

## Repository Architecture (from exploration)

```
codex/
├── codex-rs/           ← Rust core (130 crates)
│   ├── core/           ← Main agent orchestration
│   ├── agent-graph-store/  ← Agent state/memory
│   ├── agent-roles/    ← Agent role definitions
│   ├── sandboxing/     ← Sandbox execution
│   ├── exec-server/    ← Execution server
│   ├── exec/           ← Execution layer
│   ├── tools/          ← Tool definitions
│   ├── mcp/            ← MCP protocol
│   ├── core-plugins/   ← Plugin system
│   ├── app-server/     ← App server daemon
│   ├── codex-mcp/      ← MCP client
│   ├── message-history/ ← Conversation history
│   ├── memories/       ← Long-term memory
│   ├── rollout/        ← Feature rollouts
│   ├── model-provider/ ← Model abstraction
│   ├── cli/            ← CLI interface
│   ├── codex-api/      ← API client
│   └── ... (130 total)
├── codex-cli/          ← TypeScript CLI
├── sdk/                ← SDK
├── docs/               ← Documentation
├── bazel/              ← Bazel build files
└── scripts/            ← Build/dev scripts
```

**Build System**: Bazel (primary), Cargo (per-crate)
**Key Tool**: `just` command for common tasks
**Rust Toolchain**: Defined in `codex-rs/rust-toolchain.toml`

---

## Phase-by-Phase Plan

### Phase 0: Environment Setup (Days 1–3)

#### Task 0.1 — Install Rust
- Install Rust via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Verify: `rustc --version`, `cargo --version`
- Install `just`: `cargo install just`
- Install `ripgrep`: `cargo install ripgrep`
- Add to PATH

#### Task 0.2 — Read Contributing & Architecture Docs
- Read `docs/contributing.md` ✅ (already done)
- Read `docs/install.md`
- Read `codex-rs/README.md`
- Read `AGENTS.md` ✅ (already done — key rules captured)
- Read `codex-rs/docs/` for architecture overviews

#### Task 0.3 — Build/Run Codex
- Attempt: `just build` or `cargo build --package codex-core`
- Try running: `codex` CLI
- Document what works, what doesn't

#### Task 0.4 — Create `openai-codex-lab` Repository
```
openai-codex-lab/
├── README.md
├── ARCHITECTURE.md
├── WEEKLY_LOG.md
├── /architecture-notes
│   ├── 01-agent-execution-loop.md
│   ├── 02-sandboxing.md
│   ├── 03-state-and-memory.md
│   ├── 04-tool-system.md
│   ├── 05-model-provider.md
│   ├── 06-communication-patterns.md
│   └── 07-rollout-and-eval.md
├── /issues
│   ├── issue-tracker.md
│   └── reproductions/
├── /reproductions
│   └── (logs, steps, findings)
├── /experiments
│   └── (code experiments)
├── /evals
│   └── (evaluation frameworks)
├── /rust-learning
│   ├── notes.md
│   ├── exercises/
│   └── references.md
├── /design-proposals
│   └── (technical proposals)
└── /mini-agent-runtime
    ├── src/
    ├── Cargo.toml
    └── README.md
```

---

### Phase 1: Understand the Agent Execution Loop (Week 1–2)

**Goal**: Map the complete flow from User input → Agent decision → Tool call → Execution → Observation → Model → Next action

#### Study Targets:
1. `codex-rs/core/` — Main orchestration loop
2. `codex-rs/agent-graph-store/` — State management
3. `codex-rs/message-history/` — Conversation flow
4. `codex-rs/memories/` — Long-term memory
5. `codex-rs/model-provider/` — Model abstraction
6. `codex-rs/agent-roles/` — Role definitions

#### Deliverables:
- `architecture-notes/01-agent-execution-loop.md`
- Architecture diagram (text/mermaid)
- List of key types and traits
- Understanding of the `Agent` struct and its behavior

#### Week 1 Checklist:
- [ ] Read `codex-rs/core/src/lib.rs`
- [ ] Trace the main execution loop function
- [ ] Identify how user messages enter the system
- [ ] Trace how model responses generate tool calls
- [ ] Document the message types and their flow
- [ ] Write `architecture-notes/01-agent-execution-loop.md`

#### Week 2 Checklist:
- [ ] Read `codex-rs/agent-graph-store/src/`
- [ ] Understand state persistence across turns
- [ ] Read `codex-rs/message-history/src/`
- [ ] Read `codex-rs/memories/src/`
- [ ] Understand how context is managed
- [ ] Read `codex-rs/agent-roles/src/`
- [ ] Document agent role system

---

### Phase 2: Rust Deep Dive (Weeks 2–4, parallel with Phase 1)

**Goal**: Learn enough Rust to read and understand the Codex codebase

#### Study Order (priority-based):
1. **Ownership & Borrowing** (2 days) — Critical for understanding Rust patterns
2. **Structs, Enums, Traits** (2 days) — Core type system
3. **Result & Option** (1 day) — Error handling patterns
4. **Async/Await + Tokio** (3 days) — Essential for agent runtime
5. **Modules & Visibility** (1 day) — Code organization
6. **Cargo & Workspaces** (1 day) — Build system understanding
7. **Generics & Lifetimes** (2 days) — Advanced patterns in codebase
8. **Error Handling Patterns** (2 days) — How Codex handles errors

#### Resources:
- `codex-rs/rust-toolchain.toml` — Check what Rust version is used
- `codex-rs/Cargo.toml` — Workspace dependencies (Tokio, serde, etc.)
- Study actual code patterns in the codebase

#### Deliverables:
- `rust-learning/notes.md` — Personal notes
- `rust-learning/exercises/` — Small practice programs
- Ability to read any file in `codex-rs/` without getting lost

---

### Phase 3: Deep Dive into Key Systems (Weeks 3–6)

#### 3A: Sandboxing & Isolation (Week 3)
- Study: `codex-rs/sandboxing/`, `codex-rs/exec-server/`, `codex-rs/exec/`
- Study: `codex-rs/windows-sandbox-rs/`, `codex-rs/windows-sandbox-service/`
- Study: `codex-rs/bwrap/` (bubblewrap for Linux sandboxing)
- Study: `codex-rs/process-hardening/`
- Key questions: How is code executed safely? What permissions are enforced?
- Deliverable: `architecture-notes/02-sandboxing.md`

#### 3B: Tool System (Week 4)
- Study: `codex-rs/tools/`, `codex-rs/shell-command/`, `codex-rs/mcp/`, `codex-rs/codex-mcp/`
- Study: `codex-rs/core-plugins/`, `codex-rs/connectors/`
- Key questions: How are tools defined? How are they dispatched? What's the MCP protocol integration?
- Deliverable: `architecture-notes/04-tool-system.md`

#### 3C: Communication & State (Week 5)
- Study: `codex-rs/message-history/`, `codex-rs/memories/`, `codex-rs/agent-message-board-client/`
- Study: `codex-rs/app-server/`, `codex-rs/app-server-protocol/`
- Study: `codex-rs/state/`, `codex-rs/thread-store/`
- Key questions: How does state persist? How does the app server communicate?
- Deliverable: `architecture-notes/03-state-and-memory.md`

#### 3D: Model & Rollout (Week 6)
- Study: `codex-rs/model-provider/`, `codex-rs/model-provider-info/`
- Study: `codex-rs/rollout/`, `codex-rs/rollout-trace/`
- Study: `codex-rs/chatgpt/`, `codex-rs/backend-client/`
- Key questions: How are models abstracted? How do rollouts work?
- Deliverable: `architecture-notes/05-model-provider.md` and `07-rollout-and-eval.md`

---

### Phase 4: Issue Selection & Reproduction (Weeks 6–8)

#### Task 4.1 — Browse Issues
- Visit: https://github.com/openai/codex/issues
- Filter by: "good first issue", "help wanted", "area:sandbox" / "area:tools"
- Document 10–15 potential issues in `issues/issue-tracker.md`

#### Task 4.2 — Pick One Issue
- Criteria: Small scope, clear reproduction, good documentation
- Must be an issue where you can provide: reproduction steps, logs, root-cause analysis

#### Task 4.3 — Reproduce & Analyze
- **Day 1**: Understand issue, gather all info
- **Day 2**: Set up reproduction environment
- **Day 3**: Reproduce the issue
- **Day 4**: Trace code to find root cause
- **Day 5**: Write technical analysis
- Deliverable: `issues/reproductions/ISSUE-NUMBER.md`

#### Task 4.4 — Publish Analysis
- Post findings as an issue comment or new issue
- Include: reproduction steps, logs, root-cause analysis, potential approaches
- This is the type of contribution OpenAI currently accepts

---

### Phase 5: Build Mini Agent Runtime (Weeks 8–12)

**Goal**: Build your own agent runtime without LangChain to deeply understand the concepts

#### Architecture:
```
User Input
  ↓
[Planner] → Determine intent and steps
  ↓
[Tool Registry] → Available tools
  ↓
[Executor] → Execute tools in sandbox
  ↓
[Observer] → Capture results
  ↓
[State Manager] → Track progress
  ↓
[Model Interface] → LLM for reasoning
  ↓
[Loop back to Planner]
```

#### Features to implement:
1. Basic tool execution (file ops, web, shell)
2. Timeout and retry logic
3. Permission system
4. Max steps limit
5. Cost tracking (token estimation)
6. Structured logging
7. Evaluation framework
8. State persistence

#### Deliverable: `mini-agent-runtime/` with working Rust or Python implementation

---

### Phase 6: Public Engineering Record (Weeks 12–16)

#### Task 6.1 — Consolidate All Notes
- Ensure all `architecture-notes/` are complete
- Update `ARCHITECTURE.md`
- Write comprehensive `WEEKLY_LOG.md`

#### Task 6.2 — Create GitHub Presence
- Push `openai-codex-lab` to your GitHub
- Write blog posts (dev.to, medium, or GitHub Pages)
- Create issue analyses with root-cause findings

#### Task 6.3 — Engage with Community
- Comment on openai/codex issues with your findings
- Share your mini-agent-runtime project
- Connect with relevant maintainers

#### Task 6.4 — Prepare Application Materials
- Update CV with specific technical achievements
- Document all architecture knowledge gained
- Prepare interview talking points

---

## Weekly Tracking Template

```markdown
## Week N (Date Range)

### Goals
- [ ] Goal 1
- [ ] Goal 2

### Progress
- What I read:
- What I traced:
- What I learned:

### Key Findings
- Finding 1
- Finding 2

### Blockers
- Blocker 1

### Next Week
- Plan for next week
```

---

## Metrics for Success

**NOT commits. NOT lines of code.**

✅ Can I explain the agent execution loop from memory?
✅ Can I trace a bug from symptom to root cause in the code?
✅ Can I reproduce and analyze an open issue?
✅ Can I explain how sandboxing works in Codex?
✅ Can I explain how tools are dispatched?
✅ Can I explain what my Mini Agent Runtime does and why it differs from Codex?
✅ Can I write a technical analysis that would be valuable to the OpenAI team?

---

## Quick Reference

| Resource | Location |
|----------|----------|
| Contributing | `codex/docs/contributing.md` |
| Install Guide | `codex/docs/install.md` |
| Rust Toolchain | `codex-rs/rust-toolchain.toml` |
| Core Crate | `codex-rs/core/` |
| Sandboxing | `codex-rs/sandboxing/` |
| Tools | `codex-rs/tools/` |
| Agent Roles | `codex-rs/agent-roles/` |
| Messages | `codex-rs/message-history/` |
| Memory | `codex-rs/memories/` |
| State | `codex-rs/state/` |
| App Server | `codex-rs/app-server/` |
| MCP | `codex-rs/mcp/` |
| Exec Server | `codex-rs/exec-server/` |
| Issues | `https://github.com/openai/codex/issues` |
