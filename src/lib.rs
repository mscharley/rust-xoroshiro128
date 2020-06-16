//! Utilities for generating psuedo-random numbers quickly.

#![cfg_attr(not(feature = "std"), no_std)]

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://mscharley.github.io/rust-xoroshiro128/")]

#[cfg(feature = "rand")]
extern crate rand;
extern crate rand_core;

pub use rand_core::{SeedableRng, RngCore};

mod xoroshiro;
mod xorshift;
mod splitmix;

/// An `Rng` that can jump forward to provide multiple concurrent streams of numbers.
///
/// This trait is intended for use with parallel computations. You can initialise a single `Rng`
/// instance then jump for each new thread.
///
/// The precise number of jumps that will be available before cycling back to the initial starting
/// point is implementation specific but is intended to be the square root of the period of the Rng.
/// This will provide `x` number of threads with `x` number of random numbers available to them.
pub trait JumpableRng: Rng {
  /// Jump forward in the sequence of random numbers.
  ///
  /// Jumps past a number of random integers equal to the square root of the period of the current
  /// `Rng` implementation.
  fn jump(&mut self) -> Self;
}

pub use xoroshiro::Xoroshiro128Rng;
pub use xorshift::XorShift1024Rng;
pub use splitmix::SplitMix64Rng;

#[cfg(all(test, feature = "rand"))]
mod tests;

fn u64_from_sl(s: &[u8]) -> u64 {
  u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]])
}
