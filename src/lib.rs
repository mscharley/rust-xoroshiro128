//! Utilities for generating psuedo-random numbers quickly.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://mscharley.github.io/rust-xoroshiro128/")]

extern crate rand;

pub use rand::{Rng, SeedableRng, Rand};

mod xoroshiro;
pub use xoroshiro::Xoroshiro128Rng;

mod splitmix;
pub use splitmix::SplitMix64Rng;

#[cfg(test)]
mod tests;
