//! utility used in no_std enviroment.

#![no_std]
pub mod lock;
pub mod queue;
pub mod event;


#[cfg(test)]
#[cfg(feature = "std")]
use std as core;
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lock = lock::Lock::Unlocked;
        assert_eq!(lock, lock::Lock::Unlocked);
    }
}
