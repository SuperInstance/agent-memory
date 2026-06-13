# Agent Memory

**Agent Memory** is a Rust library implementing a cognitive memory system for AI agents — episodic, semantic, and procedural memory with exponential time-decay, access reinforcement, and strength-based consolidation.

## Why It Matters

Human and animal cognition depends on multiple memory systems with distinct properties: episodic memory (what happened), semantic memory (what is known), and procedural memory (how to do things). AI agents that operate over extended time horizons face the same challenge: without a memory system, every interaction starts from scratch. This library implements the computational analog of these cognitive subsystems with biologically inspired dynamics — memories that decay over time unless reinforced through retrieval, and a consolidation process that forgets irrelevant memories to prevent unbounded growth. The decay-reinforcement model ensures that frequently accessed, high-importance memories persist while stale low-value memories are pruned.

## How It Works

Each memory entry stores: content, category, importance weight (0.0–1.0), timestamp, tags, and access count. The core innovation is the **memory strength** function:

```
S(id) = importance(id) × reinforcement(id) × decay(id)

reinforcement = 1 + ln(1 + access_count)    // logarithmic, diminishing returns
decay = e^(−λ × age_hours)                    // exponential, λ = decay_rate
```

This produces three key dynamics:

1. **Decay:** All memories weaken exponentially. With λ = 0.01, a memory retains ~90% strength after 10 hours, ~33% after 5 days.
2. **Reinforcement:** Each retrieval increments `access_count`, boosting strength logarithmically. The first access gives +0.69 (ln 2), the tenth gives +0.10 (ln(11) − ln(10)). This mirrors the spacing effect in human memory.
3. **Importance weighting:** High-importance memories (importance = 0.9) decay slower than low-importance ones (importance = 0.3), ensuring critical knowledge survives longer.

**Consolidation** prunes memories below a threshold strength:

```
consolidate(threshold τ):
  for each memory id where S(id) < τ:
    remove(id)
  return count_removed
```

This is O(n) per consolidation pass. With n = 10,000 memories and τ = 0.05, typical pruning removes 15–30% of memories per day of operation.

**Memory categories:**

| Category | Content | Example |
|----------|---------|---------|
| Episodic | Events, experiences | "User asked about Rust at 3pm" |
| Semantic | Facts, knowledge | "Rust is memory-safe without GC" |
| Procedural | Skills, procedures | "To deploy: build → test → push" |

## Quick Start

```rust
fn main() {
    let mut mem = AgentMemory::new(0.01);
    mem.store("m1", "I learned Rust today", MemoryCategory::Episodic, 0.8, vec!["rust".into()]);
    let entry = mem.recall("m1").unwrap();
    assert_eq!(entry.content, "I learned Rust today");
    assert_eq!(entry.access_count, 1);
    let results = mem.search_by_tag("rust");
    assert_eq!(results.len(), 1);
}
```

## API

| Method | Description |
|--------|-------------|
| `AgentMemory::new` | Create with decay rate λ |
| `store` | Insert with id, content, category, importance, tags |
| `recall` | Retrieve by ID (increments access_count) |
| `search_by_tag` | Find memories matching a tag |
| `search_by_content` | Full-text keyword search |
| `memory_strength` | Current S(id) value |
| `consolidate` | Prune memories below threshold |
| `ranked_memories` | IDs sorted by strength descending |

## Architecture Notes

Agent Memory provides the **persistent cognitive substrate** for SuperInstance agents. Within γ + η = C, the memory system stores the conservation-law observations (γ-layer episodic events) that the η-layer intelligence reasons over. The decay-reinforcement dynamics ensure that frequently observed conservation patterns persist while transient anomalies fade, mirroring how the avoidance ratio (Law 5) stabilizes across scales.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Tulving, E. (1972). "Episodic and Semantic Memory." In *Organization of Memory*, Academic Press.
2. Anderson, J.R. & Schooler, L.J. (1991). "Reflections of the Environment in Memory." *Psychological Science*, 2(6), 396–408.
3. Ebbinghaus, H. (1885). *Memory: A Contribution to Experimental Psychology*. (Decay theory origin.)

## License

MIT
