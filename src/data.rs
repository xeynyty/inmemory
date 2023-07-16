use crate::utils::unix_epoch_now;

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
            iat: unix_epoch_now() + lifetime_sec
        }
    }

    pub fn is_alive(&self) -> bool {
        let now = unix_epoch_now();

        if self.iat < now {
            return false
        }
        true // Alive :)
    }

    pub fn get(&self) -> Option<V> {
        if self.is_alive() {
            return Some(self.base.clone())
        }
        None
    }

    pub fn unwrap(&self) -> V {
        self.base.clone()
    }

}
