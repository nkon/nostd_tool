//! Lock Library.
//!
//! # Examples
//!
//! ```
//! use nostd_tool::lock;
//! let mut l = lock::Lock::Unlocked;
//! assert_eq!(l,lock::Lock::Unlocked);
//! l.get_lock();
//! assert_eq!(l,lock::Lock::Locked);
//! l.unlock();
//! assert_eq!(l,lock::Lock::Unlocked);
//! ```

#[derive(Debug, PartialEq)]
pub enum Lock {
    Locked,
    Unlocked,
}

impl Lock {

    /// get lock.
    /// 
    /// if can not get lock, wait forever until get lock.
    /// use core::sync::atomic::AtomicBool; が、なぜか使えないので、enum Lock を実装する。
    pub fn get_lock(&mut self) -> () {
        loop {
            match *self {
                Lock::Locked => continue,
                _ => {
                    *self = Lock::Locked;
                    break;
                }
            }
        }
    }

    /// unlock.
    pub fn unlock(&mut self) -> () {
        *self = Lock::Unlocked;
    }
}

