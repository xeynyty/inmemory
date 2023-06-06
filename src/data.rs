use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Data<V> {
    base: V,
    /// Time, when it will die
    iat: u64
}

impl<V> Data<V>
where
    V: Clone
{

    pub fn new(base: V, lifetime_sec: u64) -> Self {
        Data {
            base,
            iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + lifetime_sec
        }
    }

    pub fn is_alive(&self) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        if self.iat < now {
            return false
        }
        true // Alive :)
    }

    pub fn unwrap(&self) -> V {
        self.base.clone()
    }

}