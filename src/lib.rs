//! Utilities for generating psuedo-random numbers quickly.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://mscharley.github.io/rust-xoroshiro128/")]

extern crate rand;

pub use rand::{Rng, SeedableRng, RngCore};

mod xoroshiro;
mod xorshift;
mod splitmix;

pub use xoroshiro::Xoroshiro128Rng;
pub use xorshift::XorShift1024Rng;
pub use splitmix::SplitMix64Rng;

#[cfg(test)]
mod tests;

fn u64_from_sl(s: &[u8]) -> u64 {
       u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]])
}
