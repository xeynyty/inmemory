use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Div;
use std::time::{Duration, SystemTime};
use tokio::time::{interval, sleep};
use crate::memory::Memory;
use crate::data::Data;

pub struct Manager {
    bytes_limit: usize,
}

impl Manager {

    /// New Manager
    pub fn new() -> Self {
        Manager {
            bytes_limit: 0
        }
    }

    /// Change gc limit
    pub fn limit(self, bytes: usize) -> Self {
        Self {
            bytes_limit: bytes,
            ..self
        }
    }

    /// Run GC and return Memory
    pub async fn run<K, V>(&self, gc_interval: Option<u64>) -> Memory<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + Clone + Debug + 'static,
    {

        let limit = match self.bytes_limit {
            0 => 0,
            x => x / std::mem::size_of::<Data<V>>(),
        };

        let interval = match gc_interval {
            Some(x) => x,
            None => 60 * 60
        };

        let (tx, mut rx) = tokio::sync::mpsc::channel::<K>(100);
        let service = Memory::new(limit, Some(tx));

        let s = service.clone();
        tokio::spawn(async move {
            let service = s;

            let mut old_clear = SystemTime::now();

            loop {
                if let Some(r_key) = rx.recv().await {
                    service.remove(r_key).await;
                }

                if old_clear.elapsed().unwrap().as_secs() > interval {
                    service.clear().await;
                    old_clear = SystemTime::now();
                }
            }
        });
        service
    }

}