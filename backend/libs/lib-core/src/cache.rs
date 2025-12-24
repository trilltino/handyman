//! # Simple Cache
//!
//! In-memory caching for frequently accessed data.
//!
//! ## Usage
//!
//! ```rust
//! use lib_core::cache::{Cache, CacheConfig};
//!
//! let cache: Cache<String, User> = Cache::new(CacheConfig::default());
//! cache.set("user:1".to_string(), user, Duration::from_secs(300));
//! let user = cache.get(&"user:1".to_string());
//! ```

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Cache entry with expiration.
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

impl<V> CacheEntry<V> {
    fn new(value: V, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: Instant::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

/// Cache configuration.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Default time-to-live for entries
    pub default_ttl: Duration,
    /// Maximum number of entries (0 = unlimited)
    pub max_entries: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl: Duration::from_secs(300), // 5 minutes
            max_entries: 10000,
        }
    }
}

/// Thread-safe in-memory cache.
#[derive(Clone)]
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    entries: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    config: CacheConfig,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a new cache with the given configuration.
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Gets a value from the cache.
    ///
    /// Returns `None` if the key doesn't exist or is expired.
    pub fn get(&self, key: &K) -> Option<V> {
        let entries = self.entries.read().ok()?;

        if let Some(entry) = entries.get(key) {
            if !entry.is_expired() {
                return Some(entry.value.clone());
            }
        }

        None
    }

    /// Sets a value in the cache with the default TTL.
    pub fn set(&self, key: K, value: V) {
        self.set_with_ttl(key, value, self.config.default_ttl);
    }

    /// Sets a value in the cache with a custom TTL.
    pub fn set_with_ttl(&self, key: K, value: V, ttl: Duration) {
        if let Ok(mut entries) = self.entries.write() {
            // Evict expired entries if we're at capacity
            if self.config.max_entries > 0 && entries.len() >= self.config.max_entries {
                entries.retain(|_, v| !v.is_expired());
            }

            entries.insert(key, CacheEntry::new(value, ttl));
        }
    }

    /// Removes a value from the cache.
    pub fn remove(&self, key: &K) -> Option<V> {
        if let Ok(mut entries) = self.entries.write() {
            return entries.remove(key).map(|e| e.value);
        }
        None
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }
    }

    /// Returns the number of entries in the cache (including expired).
    pub fn len(&self) -> usize {
        self.entries.read().map(|e| e.len()).unwrap_or(0)
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes all expired entries.
    pub fn cleanup_expired(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.retain(|_, v| !v.is_expired());
        }
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub entries: usize,
}

impl CacheStats {
    /// Calculates hit ratio.
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_set_get() {
        let cache: Cache<String, i32> = Cache::new(CacheConfig::default());
        cache.set("key1".to_string(), 42);

        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.get(&"key2".to_string()), None);
    }

    #[test]
    fn test_cache_expiration() {
        let config = CacheConfig {
            default_ttl: Duration::from_millis(10),
            max_entries: 100,
        };
        let cache: Cache<String, i32> = Cache::new(config);
        cache.set("key1".to_string(), 42);

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(20));

        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_cache_remove() {
        let cache: Cache<String, i32> = Cache::new(CacheConfig::default());
        cache.set("key1".to_string(), 42);

        let removed = cache.remove(&"key1".to_string());
        assert_eq!(removed, Some(42));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }
}
