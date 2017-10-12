//! utility used in no_std enviroment.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod lock;
pub mod queue;


#[cfg(test)]
#[cfg(feature = "std")]
use std as core;
mod tests {
    #[test]
    fn it_works() {
    }
}
