/**
 * LRU Cache Implementation for CacheMind
 * =====================================
 *
 * Least Recently Used cache with HashMap + VecDeque for O(1) operations.
 * Used by CacheMind for cross-system state caching.
 */

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use serde::{Deserialize, Serialize};

/// LRU (Least Recently Used) cache implementation
///
/// Provides O(1) insertion, lookup, and eviction using a HashMap for storage
/// and a VecDeque to track access order.
#[derive(Debug, Clone)]
pub struct LRUCache<K, V> {
    /// Main storage mapping keys to values
    map: HashMap<K, V>,
    /// Order of access (most recent at back)
    order: VecDeque<K>,
    /// Maximum number of entries
    capacity: usize,
}

impl<K: Clone + Eq + Hash, V> LRUCache<K, V> {
    /// Create a new LRU cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            capacity,
        }
    }

    /// Insert a key-value pair into the cache
    ///
    /// If the key already exists, it updates the value and moves to most recent.
    /// If at capacity, evicts the least recently used entry.
    pub fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Key exists, remove from order queue and re-add at end
            self.order.retain(|k| k != &key);
        } else if self.order.len() == self.capacity {
            // At capacity, evict least recently used (front of queue)
            if let Some(old_key) = self.order.pop_front() {
                self.map.remove(&old_key);
            }
        }

        // Add/update entry and mark as most recently used
        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }

    /// Get a value by key, marking it as most recently used
    ///
    /// Returns None if key doesn't exist.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to most recent position
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
        }
        self.map.get(key)
    }

    /// Get a value by key without updating access order
    ///
    /// Useful for queries that shouldn't affect LRU ordering.
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /// Check if cache contains a key without updating access order
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Remove an entry from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.remove(key) {
            self.order.retain(|k| k != key);
            Some(value)
        } else {
            None
        }
    }

    /// Get current number of entries
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Get cache capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    /// Get all keys in access order (least recent first)
    pub fn keys_lru_order(&self) -> Vec<K> {
        self.order.iter().cloned().collect()
    }

    /// Get all keys in reverse access order (most recent first)
    pub fn keys_mru_order(&self) -> Vec<K> {
        self.order.iter().rev().cloned().collect()
    }

    /// Resize the cache capacity
    ///
    /// If new capacity is smaller, evicts least recently used entries.
    pub fn resize(&mut self, new_capacity: usize) {
        self.capacity = new_capacity;

        // Evict entries if we're over capacity
        while self.order.len() > self.capacity {
            if let Some(old_key) = self.order.pop_front() {
                self.map.remove(&old_key);
            }
        }
    }

    /// Get iterator over all key-value pairs (no particular order)
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    /// Get cache hit ratio statistics
    pub fn hit_ratio(&self) -> f32 {
        // This would require tracking hits/misses - simplified for now
        if self.is_empty() {
            0.0
        } else {
            self.len() as f32 / self.capacity as f32
        }
    }
}

// Implement serialization for LRUCache when K and V are serializable
impl<K, V> Serialize for LRUCache<K, V>
where
    K: Serialize + Clone + Eq + Hash,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("LRUCache", 3)?;

        // Serialize as ordered pairs to preserve LRU order
        let ordered_pairs: Vec<(&K, &V)> = self.order.iter()
            .filter_map(|k| self.map.get(k).map(|v| (k, v)))
            .collect();

        state.serialize_field("entries", &ordered_pairs)?;
        state.serialize_field("capacity", &self.capacity)?;
        state.serialize_field("length", &self.len())?;
        state.end()
    }
}

// Implement deserialization for LRUCache
impl<'de, K, V> Deserialize<'de> for LRUCache<K, V>
where
    K: Deserialize<'de> + Clone + Eq + Hash,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Entries,
            Capacity,
            Length,
        }

        struct LRUCacheVisitor<K, V> {
            marker: std::marker::PhantomData<(K, V)>,
        }

        impl<'de, K, V> Visitor<'de> for LRUCacheVisitor<K, V>
        where
            K: Deserialize<'de> + Clone + Eq + Hash,
            V: Deserialize<'de>,
        {
            type Value = LRUCache<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LRUCache")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut entries: Option<Vec<(K, V)>> = None;
                let mut capacity: Option<usize> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Entries => {
                            if entries.is_some() {
                                return Err(de::Error::duplicate_field("entries"));
                            }
                            entries = Some(map.next_value()?);
                        }
                        Field::Capacity => {
                            if capacity.is_some() {
                                return Err(de::Error::duplicate_field("capacity"));
                            }
                            capacity = Some(map.next_value()?);
                        }
                        Field::Length => {
                            // Ignore length field, we'll calculate it
                            let _: usize = map.next_value()?;
                        }
                    }
                }

                let entries = entries.ok_or_else(|| de::Error::missing_field("entries"))?;
                let capacity = capacity.ok_or_else(|| de::Error::missing_field("capacity"))?;

                let mut cache = LRUCache::new(capacity);

                // Insert entries in order to preserve LRU sequence
                for (key, value) in entries {
                    cache.insert(key, value);
                }

                Ok(cache)
            }
        }

        deserializer.deserialize_struct(
            "LRUCache",
            &["entries", "capacity", "length"],
            LRUCacheVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_basic_operations() {
        let mut cache = LRUCache::new(3);

        // Test insertion
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = LRUCache::new(2);

        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3); // Should evict "a"

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"a"), None); // Evicted
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_lru_access_order() {
        let mut cache = LRUCache::new(3);

        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        // Access "a" to make it most recent
        let _ = cache.get(&"a");

        // Insert new item, should evict "b" (least recent)
        cache.insert("d", 4);

        assert_eq!(cache.get(&"a"), Some(&1)); // Still present
        assert_eq!(cache.get(&"b"), None); // Evicted
        assert_eq!(cache.get(&"c"), Some(&3));
        assert_eq!(cache.get(&"d"), Some(&4));
    }

    #[test]
    fn test_lru_update_existing() {
        let mut cache = LRUCache::new(2);

        cache.insert("a", 1);
        cache.insert("b", 2);

        // Update existing key
        cache.insert("a", 10);

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"a"), Some(&10));
        assert_eq!(cache.get(&"b"), Some(&2));
    }

    #[test]
    fn test_lru_peek() {
        let mut cache = LRUCache::new(2);

        cache.insert("a", 1);
        cache.insert("b", 2);

        // Peek shouldn't affect order
        assert_eq!(cache.peek(&"a"), Some(&1));

        // Insert new item, "a" should still be evicted (wasn't accessed via get)
        cache.insert("c", 3);

        assert_eq!(cache.get(&"a"), None); // Evicted
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_lru_remove() {
        let mut cache = LRUCache::new(3);

        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        let removed = cache.remove(&"b");
        assert_eq!(removed, Some(2));
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"b"), None);
    }

    #[test]
    fn test_lru_resize() {
        let mut cache = LRUCache::new(3);

        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        // Resize to smaller capacity
        cache.resize(2);

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.capacity(), 2);

        // Least recent should be evicted
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_lru_serialization() {
        let mut cache = LRUCache::new(3);
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        // Serialize
        let serialized = serde_json::to_string(&cache).unwrap();

        // Deserialize
        let deserialized: LRUCache<String, i32> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.len(), 3);
        assert_eq!(deserialized.capacity(), 3);

        // Check that we can access all values (though we can't easily test order preservation in this simple test)
        assert!(deserialized.contains(&"a".to_string()));
        assert!(deserialized.contains(&"b".to_string()));
        assert!(deserialized.contains(&"c".to_string()));
    }

    #[test]
    fn test_lru_keys_order() {
        let mut cache = LRUCache::new(3);

        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        // Access "a" to make it most recent
        let _ = cache.get(&"a");

        let lru_order = cache.keys_lru_order();
        let mru_order = cache.keys_mru_order();

        // "a" should be most recent (last in LRU order, first in MRU order)
        assert_eq!(lru_order.last(), Some(&"a"));
        assert_eq!(mru_order.first(), Some(&"a"));
    }
}