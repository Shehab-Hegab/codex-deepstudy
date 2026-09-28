# 🎯 90-Day OpenAI AI Engineer Apprenticeship Plan

> **Goal**: Build demonstrable expertise in AI agent systems through deep Codex source-code study, open-source contributions, and public engineering record — positioning for OpenAI candidacy.
>
> **Status**: Branch `shehab-codex-deepstudy` created at https://github.com/Shehab-Hegab/codex-deepstudy

---

## ⚠️ Contribution Strategy (Critical)

OpenAI does **NOT** accept external PRs to `openai/codex`. Contributions must target:

| Contribution Type | How | Impact |
|---|---|---|
| **High-quality issues** | File on `openai/codex` with reproducible bugs, architecture analysis | ✅ Visible to maintainers |
| **Open-source tools** | Build Rust agent frameworks, CLI tools, integrations | ✅ Shows engineering depth |
| **Technical analysis** | Blog posts, GitHub repos, architecture deep-dives | ✅ Establishes authority |
| **OpenAI ecosystem** | Tools for Codex API, MCP servers, OpenAI SDK extensions | ✅ Adjacent to OpenAI work |
| **Public record** | Weekly logs, architecture notes, design proposals | ✅ Demonstrates process |

---

## 📅 MONTH 1: Deep Study + Rust Foundation + Issue Filing (Days 1–30)

### Week 1: Complete Source Code Mapping
- [x] ✅ Clone `openai/codex` (shallow, depth 1) — **DONE**
- [x] ✅ Install Rust 1.98.1 + `just` + `ripgrep` — **DONE**
- [x] ✅ Read `codex-core` crate: `lib.rs`, `agent/`, `session/`, `exec/`, `guardian/` — **DONE**
- [x] ✅ Read `codex-protocol` crate: `turn_input.rs`, `items.rs`, `approvals.rs`, `protocol.rs` — **DONE**
- [x] ✅ Create architecture notes (`01-agent-execution-loop.md`) — **DONE**
- [ ] Read `thread_manager.rs`, `rollout/`, `codex_thread.rs`, `realtime_conversation/`
- [ ] Read `codex-rs/exec-server/src/` — execution server architecture
- [ ] Read `sandboxing/src/` — Linux/Windows sandbox implementation

### Week 2: Rust Language Deep Dive
- [ ] Rust ownership & borrowing — 40 exercises
- [ ] Async Rust: `tokio`, `async/await`, `Pin`, `Future` trait
- [ ] Rust modules, crates, `pub` visibility, `mod.rs` patterns
- [ ] Error handling: `Result`, `Option`, custom error types
- [ ] Trait objects, generics, lifetime annotations
- [ ] Read Rust Book (https://doc.rust-lang.org/book/) — chapters 1-15
- [ ] Practice: implement a small async runtime in Rust

### Week 3: Codex Deep Dive — Execution & Guardian
- [ ] Trace `TurnInput → TurnContext → exec → sandbox` full flow
- [ ] Read `exec.rs` (1276 lines) line-by-line with notes
- [ ] Read `spawn.rs` (137 lines) — child process management
- [ ] Read `guardian/mod.rs` (210 lines) — approval/review flow
- [ ] Read `session/turn_context.rs` (1392 lines) — turn lifecycle
- [ ] Read `session/turn_input.rs` (780 lines) — input processing
- [ ] Document the **complete execution pipeline** in a new architecture note

### Week 4: Issue Filing + Reproduction
- [ ] Browse `openai/codex` issues labeled `good first issue`, `help wanted`
- [ ] Identify 3–5 issues where analysis/reproduction adds value
- [ ] Reproduce 1–2 bugs with detailed repro steps
- [ ] File **2 high-quality issues** on `openai/codex`:
  - Architecture analysis issue (e.g., thread lifecycle edge case)
  - Sandbox behavior observation with repro steps
- [ ] Create `issues/` directory in `codex-deepstudy` with repro steps

---

## 📅 MONTH 2: Build + Analysis + Ecosystem Contribution (Days 31–60)

### Week 5: Build Mini Agent Runtime
- [ ] Initialize `codex-deepstudy/mini-agent-runtime/` as Rust crate
- [ ] Implement core: `Agent` trait, `TurnContext`, `ToolRegistry`
- [ ] Implement: `AgentControl` trait (identity, spawn, send, interrupt)
- [ ] Implement: `AgentExecutionGuard` (permit-based limiting)
- [ ] Implement: `AgentRegistry` (thread management, metadata)
- [ ] Reference Codex source but build from scratch (no copy-paste)
- [ ] Write tests for each component
- [ ] Document design decisions in `design-proposals/`

### Week 6: Mini Agent Runtime — Tool Execution & Sandbox
- [ ] Implement tool execution engine with sandbox support
- [ ] Implement: `ExecParams`, `SandboxCommand`, sandbox types
- [ ] Implement: `GuardianReview` system (permission checking)
- [ ] Integrate with `codex-mcp` or build MCP client
- [ ] Add async execution with `tokio`
- [ ] Create `exec/` module matching Codex patterns
- [ ] Write integration tests

### Week 7: Open-Source Tool — Codex Analysis Toolkit
- [ ] Build `codex-analyze`: CLI tool to analyze Codex `.codex` config files
- [ ] Build `codex-mcp-bridge`: MCP server that bridges Codex tools
- [ ] Build `codex-agent-tester`: Framework for testing agent behaviors
- [ ] Publish all tools on crates.io
- [ ] Create documentation and examples
- [ ] Target: **3 published crates** by end of month

### Week 8: Technical Analysis & Blog Posts
- [ ] Write "Inside OpenAI Codex: Agent Execution Loop" — technical blog post
- [ ] Write "Rust Concurrency Patterns in OpenAI Codex" — analysis
- [ ] Write "How OpenAI Codex's Guardian System Works" — deep dive
- [ ] Publish on dev.to, medium, or personal blog
- [ ] Create `design-proposals/` with 2 formal design proposals for Codex improvements
- [ ] File 2 more issues on `openai/codex` based on analysis

---

## 📅 MONTH 3: Public Engineering Record + Applications (Days 61–90)

### Week 9: Public Engineering Record
- [ ] Write comprehensive `ARCHITECTURE.md` in `codex-deepstudy`
- [ ] Create `WEEKLY-LOGS/` with detailed entries for all 9 weeks
- [ ] Write `COMPARISON.md`: Codex vs. your Mini Agent Runtime
- [ ] Create `CONTRIBUTION-MAP.md`: What you can contribute to OpenAI ecosystem
- [ ] Record a 15-minute video or livestream walking through Codex architecture
- [ ] Share weekly logs on Twitter/X with #OpenAI #Rust #Agents

### Week 10: OpenAI Ecosystem Contributions
- [ ] Build tools for OpenAI API ecosystem:
  - `codex-sdk-extensions`: OpenAI SDK extensions for agent workflows
  - `codex-types`: Rust type definitions for OpenAI API responses
  - `codex-mcp-tools`: MCP tool integrations for Codex
- [ ] Contribute documentation to `platformlab/openai-docs` or similar
- [ ] Build a `codex-agent-framework` crate that wraps OpenAI APIs with Codex-like patterns
- [ ] Target: **5+ published crates**, all with tests and docs

### Week 11: Portfolio & Interview Prep
- [ ] Polish all GitHub repos: README, badges, CI, examples
- [ ] Create a portfolio site: `shehab-hegab.dev` or `codex-deepstudy.dev`
- [ ] Prepare 3-5 talking points about Codex architecture
- [ ] Practice explaining agent execution flow, guardian system, sandbox design
- [ ] Build a "Day in the Life of a Codex Agent" demo app
- [ ] Get feedback from community on Reddit (r/rust, r/LocalLLaMA, r/OpenAI)

### Week 12: Application & Network
- [ ] Update LinkedIn highlighting Codex deep-study, Rust expertise, agent systems
- [ ] Reach out to OpenAI engineers on Twitter/LinkedIn about your work
- [ ] Apply to OpenAI AI Engineer roles with codex-deepstudy as portfolio centerpiece
- [ ] Prepare whitepaper: "Agent Execution Systems: Lessons from OpenAI Codex"
- [ ] Final push: contribute 5+ issues/reports to OpenAI repos
- [ ] Submit one final high-quality analysis to OpenAI's issue tracker

---

## 📊 Success Metrics

| Metric | Month 1 Target | Month 2 Target | Month 3 Target |
|--------|---------------|----------------|----------------|
| Architecture notes | 1 | 4+ | 8+ |
| Issues filed on openai/codex | 2 | 4 | 6+ |
| Published Rust crates | 0 | 3 | 5+ |
| Blog posts | 0 | 3 | 5+ |
| GitHub stars (total) | 10 | 50 | 100+ |
| Weekly logs | 4 | 8 | 12 |
| Design proposals | 0 | 2 | 3+ |

---

## 🔑 Key Principles

1. **Build, don't just read** — Every week must produce code
2. **Public by default** — Everything goes on GitHub, everything is documented
3. **Quality over quantity** — One deep issue report > 10 shallow PRs
4. **Reference but don't copy** — Study Codex patterns, build your own implementation
5. **Network visibly** — Twitter, blog posts, Reddit discussions
6. **Consistency** — Daily commits, weekly logs, never skip a week

---

## 📁 Repository Structure (`codex-deepstudy`)

```
codex-deepstudy/
├── README.md                    # Project overview + 90-day plan
├── ARCHITECTURE.md              # Complete Codex architecture analysis
├── PLAN.md                      # Original execution plan
├── CONTRIBUTION-MAP.md          # What you can contribute to OpenAI
├── 01-agent-execution-loop.md   # Architecture deep-dive note
├── WEEKLY-LOGS/
│   ├── week-1.md
│   ├── week-2.md
│   ├── ...
│   └── week-12.md
├── design-proposals/
│   ├── 01-thread-lifecycle-improvements.md
│   ├── 02-guardian-review-optimizations.md
│   └── 03-sandbox-parallel-execution.md
├── issues/
│   ├── repro-steps/
│   └── analysis-reports/
├── mini-agent-runtime/          # Week 5-6: Built from scratch
│   ├── Cargo.toml
│   ├── src/
│   │   ├── agent.rs             # Agent trait, LocalAgentControl
│   │   ├── registry.rs          # AgentRegistry
│   │   ├── execution.rs         # Exec engine + sandbox
│   │   ├── guardian.rs          # Guardian review system
│   │   ├── tools.rs             # Tool registry
│   │   └── lib.rs
│   └── tests/
├── codex-analyze/               # Week 7: CLI tool
├── codex-mcp-bridge/            # Week 7: MCP server
├── codex-agent-tester/          # Week 7: Testing framework
├── codex-sdk-extensions/        # Week 10: OpenAI SDK extensions
├── codex-types/                 # Week 10: Rust type definitions
├── codex-mcp-tools/             # Week 10: MCP integrations
└── codex-agent-framework/       # Week 10: Full agent framework
```

---

## 🚀 Immediate Next Steps (Today)

```bash
# 1. Commit initial architecture notes
cd C:\OpenAI\codex-deepstudy
git add . && git commit -m "feat: initial architecture deep-dive of OpenAI Codex"
git push -u origin main

# 2. Create weekly log template
# 3. Start reading thread_manager.rs and session/mod.rs
# 4. Begin Rust Book chapters 1-5
```

---

*This plan is compressed for execution. Every task produces a commit. Every commit pushes toward OpenAI candidacy.*
