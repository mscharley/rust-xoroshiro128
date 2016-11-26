use std::io;
use {Rng, SeedableRng, Rand};

pub struct SplitMix64Rng {
  state: u64
}

impl SplitMix64Rng {
  /// Creates a new SplitMix64Rng instance which is randomly seeded.
  ///
  /// # Errors
  ///
  /// When created this way the initial values of this Rng are seeded using an OsRng. If this
  /// fails then this method will pass through the error from OsRng.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use xoroshiro128::{Rng, SplitMix64Rng};
  ///
  /// let rng = SplitMix64Rng::new();
  /// let x: u32 = rng.unwrap().gen();
  /// println!("{}", x);
  /// ```
  pub fn new() -> io::Result<Self> {
    ::rand::OsRng::new().map(|mut x: ::rand::OsRng| x.gen::<Self>())
  }
}

impl Rng for SplitMix64Rng {
  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }

  fn next_u64(&mut self) -> u64 {
    self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z: u64 = self.state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
  }
}

impl SeedableRng<u64> for SplitMix64Rng {
  fn reseed(&mut self, seed: u64) {
    self.state = seed
  }

  fn from_seed(seed: u64) -> Self {
    SplitMix64Rng { state: seed }
  }
}

/// Seed a SplitMix64Rng based on an OsRng.
///
/// # Panics
///
/// If OsRng is unavailable then this will panic. A safer option is `SplitMix64Rng::new()`
impl SeedableRng<()> for SplitMix64Rng {
  fn reseed(&mut self, _: ()) {
    self.reseed(::rand::OsRng::new().unwrap().gen::<u64>())
  }

  fn from_seed(_: ()) -> Self {
    ::rand::OsRng::new().unwrap().gen::<Self>()
  }
}

impl Rand for SplitMix64Rng {
  fn rand<R: Rng>(rng: &mut R) -> SplitMix64Rng {
    SplitMix64Rng::from_seed(rng.gen::<u64>())
  }
}
