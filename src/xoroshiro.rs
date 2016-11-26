use std::io;
use {Rng, SeedableRng, Rand};

pub struct Xoroshiro128Rng {
  state: [u64; 2]
}

impl Xoroshiro128Rng {
  /// Creates a new `Xoroshiro128Rng` instance which is randomly seeded.
  ///
  /// # Errors
  ///
  /// When created this way the initial values of this `Rng` are seeded using an `OsRng`. If this
  /// fails then this method will pass through the error from `OsRng`.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use xoroshiro128::{Rng, Xoroshiro128Rng};
  ///
  /// let rng = Xoroshiro128Rng::new();
  /// let x: u32 = rng.unwrap().gen();
  /// println!("{}", x);
  /// ```
  pub fn new() -> io::Result<Self> {
    ::rand::OsRng::new().map(|mut x: ::rand::OsRng| x.gen::<Self>())
  }

  /// Creates a new `Xoroshiro128Rng` instance which is not seeded.
  ///
  /// The initial values of this Rng are constants, so all generators created by this function
  /// will yield the same stream of random numbers. It is highly recommended that this is created
  /// through `SeedableRng` instead of this function.
  pub fn new_unseeded() -> Self {
    Xoroshiro128Rng::from_seed([
      0x193a6754a8a7d469,
      0x97830e05113ba7bb
    ])
  }
}

#[inline(always)]
fn rotate_left(x: u64, k: i32) -> u64 {
  (x << k) | (x >> (64 - k))
}

impl Rng for Xoroshiro128Rng {
  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }

  fn next_u64(&mut self) -> u64 {
    let s0: u64 = self.state[0];
    let mut s1: u64 = self.state[1];
    let result: u64 = s0.wrapping_add(s1);

    s1 ^= s0;
    self.state[0] = rotate_left(s0, 55) ^ s1 ^ (s1 << 14); // a, b
    self.state[1] = rotate_left(s1, 36); // c

    result
  }
}

/// Seed a `Xoroshiro128Rng` with a given seed.
///
/// # Panics
///
/// `Xoroshiro128Rng` is undefined for the seed `[0, 0]` and will panic if this seed is provided.
impl SeedableRng<[u64; 2]> for Xoroshiro128Rng {
  fn reseed(&mut self, seed: [u64; 2]) {
    assert!(seed != [0, 0], "Invalid seed: seed must not be 0.");
    self.state = seed
  }

  fn from_seed(seed: [u64; 2]) -> Self {
    assert!(seed != [0, 0], "Invalid seed: seed must not be 0.");
    Xoroshiro128Rng { state: seed }
  }
}

#[inline(always)]
fn splitmix_seed(seed: u64) -> [u64; 2] {
  ::SplitMix64Rng::from_seed(seed).gen()
}

impl SeedableRng<u64> for Xoroshiro128Rng {
  fn reseed(&mut self, seed: u64) {
    self.reseed(splitmix_seed(seed))
  }

  fn from_seed(seed: u64) -> Self {
    Xoroshiro128Rng::from_seed(splitmix_seed(seed))
  }
}

/// Seed a `Xoroshiro128Rng` based on an `OsRng`.
///
/// # Panics
///
/// If `OsRng` is unavailable then this will panic. A safer option is `Xoroshiro128Rng::new()`
impl SeedableRng<()> for Xoroshiro128Rng {
  fn reseed(&mut self, _: ()) {
    self.reseed(::rand::OsRng::new().unwrap().gen::<[u64; 2]>())
  }

  fn from_seed(_: ()) -> Self {
    ::rand::OsRng::new().unwrap().gen::<Self>()
  }
}

impl Rand for Xoroshiro128Rng {
  fn rand<R: Rng>(rng: &mut R) -> Xoroshiro128Rng {
    let mut seed: [u64; 2] = rng.gen();
    while seed == [0, 0] {
      seed = rng.gen();
    }

    Xoroshiro128Rng::from_seed(seed)
  }
}
