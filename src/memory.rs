use std::collections::HashMap;
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use crate::data::Data;

#[derive(Clone)]
pub struct Memory<K, V> {
    base: Arc<RwLock<HashMap<K, Data<V>>>>,
    // The limit of the number of saved records,
    // calculated from the specified number of bytes in the Manager
    limit: usize,
    sender: Option<Sender<K>>
}

impl<K, V> Memory<K, V>
where
    K: Eq + Hash + Clone + 'static,
    V: Send + Sync + Clone + 'static,
{

    /// Writes data to memory by the specified key.
    /// If the memory limit is specified, it will return an error if it is exceeded.
    pub async fn add(&self, key: K, value: V, iat: u64) -> Result<(), Error> {
        if self.limit > 0 {
            let read = self.base.read().await;

            if read.len() >= self.limit {
                self.clear().await;
            }

            if read.len() >= self.limit {
                return Err(Error::new(ErrorKind::WriteZero, "Limit of memory, len is max"))
            }
        }
        self.base.write().await.insert(key, Data::new(value, iat));

        Ok(())
    }

    /// Clears data from memory and returns it.
    pub async fn remove(&self, key: K) -> Option<Data<V>> {
        if let Some(data) = self.base.write().await.remove(&key) {
            return Some(data);
        }
        None
    }

    /// Getting data from memory
    pub async fn get(&self, key: K) -> Option<V> {
        if let Some(data) = self.base.read().await.get(&key) {
            if let Some(data) = data.get() {
                return Some(data)
            }
            if let Some(x) = &self.sender {
                if let Err(e) = x.send(key).await {
                    eprintln!("GC -> {}", e);
                }
            }
        }
        None
    }

    /// If there is stale data in memory (not yet garbage collected)
    /// then it will not be retrieved with this function.
    pub async fn get_dead(&self, key: K) -> Option<V> {
       Some(self.base.read().await.get(&key)?.unwrap())
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

        if dead_keys.is_empty() {
            return;
        }

        let mut hash = self.base.write().await;
        for key in dead_keys {
            hash.remove(&key);
        }
    }

    /// Return size of one data "cell" in bytes
    pub fn size_of_cell(&self) -> usize {
        std::mem::size_of::<Data<V>>()
    }
    
    pub fn new(limit: usize, sender: Option<Sender<K>>) -> Self {
        Self {
            base: Arc::new(RwLock::new(HashMap::new())),
            limit, sender
        }
    }
}
