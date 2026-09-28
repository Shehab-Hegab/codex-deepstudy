# Mini Agent Runtime

A Rust implementation of an AI agent runtime inspired by OpenAI Codex architecture.

## Overview

Built from scratch by studying OpenAI Codex source code. Implements:
- `Agent` trait with identity, spawn, send, interrupt
- `AgentExecutionGuard` — permit-based execution limiting
- `AgentRegistry` — thread management with metadata
- `ToolRegistry` — tool discovery and execution
- `GuardianReview` — permission checking and approval system
- `ExecEngine` — sandboxed command execution

## Structure

```
mini-agent-runtime/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── agent.rs          # Agent trait, LocalAgentControl
│   ├── registry.rs       # AgentRegistry
│   ├── execution.rs      # Exec engine, sandbox integration
│   ├── guardian.rs       # Guardian review system
│   ├── tools.rs          # Tool registry
│   └── types.rs          # Shared types (AgentStatus, AgentMessage, etc.)
└── tests/
    ├── agent_test.rs
    ├── registry_test.rs
    └── execution_test.rs
```
