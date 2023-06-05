use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;
use tokio::time::sleep;
use crate::memory::Memory;
use crate::data::Data;

pub struct Manager {
    interval_sec: u64,
    bytes_limit: usize
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
        K: Eq + PartialEq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + Clone + Debug + 'static,
    {

        let limit = match self.bytes_limit {
            0 => 0,
            x => x / std::mem::size_of::<Data<V>>(),
        };

        let service = Memory::new(self.interval_sec, limit);

        let gc = service.clone();

        // if gc interval = 0 -> gc off
        if gc.interval() != 0 {
            tokio::spawn(async move {

                let gc = gc;

                if gc.interval() == 0 {
                    return;
                }

                loop {
                    sleep(Duration::from_secs(gc.interval())).await;
                    gc.clear().await;
                }

            });
        }

        service
    }

}