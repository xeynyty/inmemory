use std::time::{SystemTime, UNIX_EPOCH};

#[inline]
pub(crate) fn unix_epoch_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}