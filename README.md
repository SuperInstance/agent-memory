# Agent Memory

**Agent memory** provides persistent state for AI agents — short-term (conversation context), working (scratch pad), and long-term (knowledge base) memory tiers.

## Why It Matters

Memory is what distinguishes an agent from a stateless function. Without persistence, every interaction starts from zero. Tiered memory mirrors human cognition: working memory for active reasoning, episodic memory for past experiences, semantic memory for distilled knowledge.

## How It Works

Implements a three-tier memory store: STM (bounded LRU buffer), working memory (mutable key-value), and LTM (vector-indexed for semantic retrieval). Includes consolidation: periodically, STM items are summarized and promoted to LTM.

## Usage

```toml
[dependencies]
agent-memory = "0.1.0"
```

```rust
use agent_memory;

// See examples/ directory for detailed usage
```

## API

- `MemoryEntry` (lib.rs)
- `MemoryCategory` (lib.rs)
- `AgentMemory` (lib.rs)

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
