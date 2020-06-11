//! # KVStore
//!
//! A key/value database store string pair in-memory

#![deny(missing_docs)]

use std::collections::HashMap;
use std::result;
use std::path::PathBuf;
use failure::Error;

/// alias for return type with error
pub type Result<T> = result::Result<T, Error>;

/// the kernal struct of KVStore
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Create a KvStore to store key/value pair.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore {
            store: HashMap::new(),
        })
    }

    /// Insert or update a key's value.
    ///
    /// # Example
    ///
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// store.set("foo".to_string(), "bar".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
        Ok(())
    }

    /// Get value by key, get None if key not exist.
    ///
    /// # Example
    ///
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// store.set("foo".to_string(), "bar".to_string());
    /// assert_eq!(store.get("foo".to_string()).unwrap(), "bar".to_string());
    /// assert_eq!(store.get("goo".to_string()), None);
    /// ```
    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.store.get(&key) {
            None => Ok(None),
            Some(v) => Ok(Some(v.clone())),
        }
    }

    /// Remove a key/value pair, do nothing if key not exist.
    ///
    /// # Example
    ///
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// store.set("foo".to_string(), "bar".to_string());
    /// store.remove("foo".to_string());
    /// store.remove("foo".to_string());
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        Ok(())
    }
}
