use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Div;
use std::time::{Duration, SystemTime};
use tokio::time::{interval, sleep};
use crate::memory::Memory;
use crate::data::Data;

pub struct Manager {
    interval_sec: u64,
    bytes_limit: usize,
}

impl Manager {

    /// New Manager
    pub fn new() -> Self {
        Manager {
            interval_sec: 0, // default
            bytes_limit: 0
        }
    }


    /// Change gc interval
    pub fn interval(self, interval_sec: u64) -> Self {
        Self {
            interval_sec,
            ..self
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
    pub async fn run<K, V>(&self) -> Memory<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + Clone + Debug + 'static,
    {

        let limit = match self.bytes_limit {
            0 => 0,
            x => x / std::mem::size_of::<Data<V>>(),
        };


        return match self.interval_sec {
            1.. => {
                let (tx, mut rx) = tokio::sync::mpsc::channel::<K>(100);
                let service = Memory::new(self.interval_sec, limit, Some(tx));
                let gc = service.clone();
                tokio::spawn(async move {
                    let gc = gc;

                    let mut old_clear = SystemTime::now();

                    loop {
                        if let Some(r_key) = rx.recv().await {
                            gc.remove(r_key).await;
                        }
                        if old_clear.elapsed().unwrap().as_secs() > gc.interval() {
                            gc.clear().await;
                            old_clear = SystemTime::now();
                        }
                    }
                });
                service
            }
            _ => {
                let service = Memory::new(self.interval_sec, limit, None);
                service
            },
        }
    }

}