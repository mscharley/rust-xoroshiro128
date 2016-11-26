extern crate rand;

mod xoroshiro;
pub use xoroshiro::XoroshiroRng;

mod splitmix;
pub use splitmix::SplitMixRng;

#[cfg(test)]
mod tests;
