//! # agent-memory
//! A memory system for AI agents: episodic, semantic, and procedural memory with recall.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single memory entry.
#[derive(Clone, Debug)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub category: MemoryCategory,
    pub importance: f64,
    pub timestamp: u64,
    pub tags: Vec<String>,
    pub access_count: u32,
}

/// Categories of agent memory.
#[derive(Clone, Debug, PartialEq)]
pub enum MemoryCategory {
    Episodic,   // Events and experiences
    Semantic,    // Facts and knowledge
    Procedural,  // Skills and procedures
}

/// The agent memory store.
pub struct AgentMemory {
    entries: HashMap<String, MemoryEntry>,
    decay_rate: f64,
}

impl AgentMemory {
    pub fn new(decay_rate: f64) -> Self {
        Self {
            entries: HashMap::new(),
            decay_rate,
        }
    }

    fn now_secs() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    /// Store a new memory.
    pub fn store(&mut self, id: &str, content: &str, category: MemoryCategory, importance: f64, tags: Vec<String>) {
        self.entries.insert(id.to_string(), MemoryEntry {
            id: id.to_string(),
            content: content.to_string(),
            category,
            importance,
            timestamp: Self::now_secs(),
            tags,
            access_count: 0,
        });
    }

    /// Recall a memory by exact ID.
    pub fn recall(&mut self, id: &str) -> Option<&MemoryEntry> {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.access_count += 1;
            Some(entry)
        } else {
            None
        }
    }

    /// Search memories by tag.
    pub fn search_by_tag(&self, tag: &str) -> Vec<&MemoryEntry> {
        self.entries.values()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Search memories by keyword in content.
    pub fn search_by_content(&self, query: &str) -> Vec<&MemoryEntry> {
        let query_lower = query.to_lowercase();
        self.entries.values()
            .filter(|e| e.content.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Get the effective strength of a memory (importance × access reinforcement × time decay).
    pub fn memory_strength(&self, id: &str) -> f64 {
        if let Some(e) = self.entries.get(id) {
            let age_secs = Self::now_secs().saturating_sub(e.timestamp) as f64;
            let age_hours = age_secs / 3600.0;
            let decay = (-self.decay_rate * age_hours).exp();
            let reinforcement = 1.0 + (e.access_count as f64).ln_1p();
            e.importance * reinforcement * decay
        } else {
            0.0
        }
    }

    /// Forget memories below a strength threshold.
    pub fn consolidate(&mut self, threshold: f64) -> usize {
        let weak: Vec<String> = self.entries.keys()
            .filter(|id| self.memory_strength(id) < threshold)
            .cloned()
            .collect();
        let count = weak.len();
        for id in weak {
            self.entries.remove(&id);
        }
        count
    }

    /// List all memory IDs sorted by strength (descending).
    pub fn ranked_memories(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.entries.keys().cloned().collect();
        ids.sort_by(|a, b| {
            self.memory_strength(b).partial_cmp(&self.memory_strength(a)).unwrap()
        });
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_recall() {
        let mut mem = AgentMemory::new(0.01);
        mem.store("m1", "I learned Rust today", MemoryCategory::Episodic, 0.8, vec!["rust".into()]);
        let entry = mem.recall("m1").unwrap();
        assert_eq!(entry.content, "I learned Rust today");
        assert_eq!(entry.access_count, 1);
    }

    #[test]
    fn test_search_by_tag() {
        let mut mem = AgentMemory::new(0.01);
        mem.store("m1", "fact 1", MemoryCategory::Semantic, 0.5, vec!["math".into()]);
        mem.store("m2", "fact 2", MemoryCategory::Semantic, 0.5, vec!["history".into()]);
        assert_eq!(mem.search_by_tag("math").len(), 1);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
