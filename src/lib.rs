extern crate rand;

mod xoroshiro;
pub use xoroshiro::XoroshiroRng;

#[cfg(test)]
mod tests;
