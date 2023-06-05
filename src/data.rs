use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Data<V> {
    base: V,
    lifetime_sec: u64,
    created_at: SystemTime
}

impl<V> Data<V>
where
    V: Clone
{

    pub fn new(base: V, lifetime_sec: u64) -> Self {
        Data {
            base, lifetime_sec,
            created_at: SystemTime::now()
        }
    }

    pub fn is_alive(&self) -> bool {
        if self.created_at.elapsed().unwrap().as_secs() > self.lifetime_sec {
            return false // Dead :(
        }
        true // Alive :)
    }

    pub fn unwrap(&self) -> V {
        self.base.clone()
    }

}