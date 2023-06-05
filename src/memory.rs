use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::data::Data;

#[derive(Clone)]
pub struct Memory<K, V> {
    base: Arc<RwLock<HashMap<K, Data<V>>>>,
    interval_sec: u64,

    // The limit of the number of saved records,
    // calculated from the specified number of bytes in the Manager
    limit: usize
}

impl<K, V> Memory<K, V>
where
    K: Eq + Hash + Clone + 'static,
    V: Send + Sync + Clone + 'static,
{

    /// Writes data to memory by the specified key.
    /// If the memory limit is specified, it will return an error if it is exceeded.
    pub async fn add(&self, key: K, value: V, lifetime_sec: u64) -> Result<&str, &str> {

        if self.limit > 0 {

            let read = self.base.read().await;

            if read.len() >= self.limit {
                self.clear().await;
            }

            if read.len() >= self.limit {
                return Err("Limit of memory, len is max")
            }
        }
        self.base.write().await.insert(key, Data::new(value, lifetime_sec));
        // println!("added");
        // println!("len {}", self.base.read().await.len());

        Ok("Ok")
    }

    /// Clears data from memory and returns it.
    pub async fn remove(&self, key: K) -> Option<Data<V>> {
        if let Some(data) = self.base.write().await.remove(&key) {
            return Some(data);
        }
        None
    }

    /// Getting data from memory,
    /// including "dead" ones, if they have not been cleared yet.
    pub async fn get(&self, key: K) -> Option<V> {
        if let Some(data) = self.base.read().await.get(&key) {
            return Some(data.unwrap());
        }
        None
    }

    /// If there is stale data in memory (not yet garbage collected)
    /// then it will not be retrieved with this function.
    pub async fn get_safety(&self, key: K) -> Option<V> {
        if let Some(data) = self.base.read().await.get(&key) {
            if data.is_alive() {
                return Some(data.unwrap());
            }
        }
        None
    }

    pub fn interval(&self) -> u64 {
        self.interval_sec
    }
    pub fn limit(&self) -> usize {
        self.limit
    }

    /// Manual activation of the garbage collector.
    /// Also used in automatic mode if interval is greater than 0.
    pub async fn clear(&self) {
        let dead_keys: Vec<K> = self.base.read().await.iter()
            .filter(|(_k, v)| !v.is_alive())
            .map(|(k, _v)| k.clone())
            .collect();

        // println!("dead {}", dead_keys.len());

        if dead_keys.len() < 1 {
            return;
        }

        let mut hash = self.base.write().await;
        for i in dead_keys {
            hash.remove(&i);
        }
    }

    pub fn size_of_cell(&self) -> usize {
        std::mem::size_of::<Data<V>>()
    }
    
    pub fn new(interval_sec: u64, limit: usize) -> Self {
        Self {
            base: Arc::new(RwLock::new(HashMap::new())),
            interval_sec, limit
        }
    }
}
