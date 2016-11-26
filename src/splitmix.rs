use std::io;
use ::rand::{Rng, SeedableRng};

pub struct SplitMix64Rng {
  state: u64,
  carry: Option<u32>
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

impl ::rand::Rng for SplitMix64Rng {
  fn next_u32(&mut self) -> u32 {
    match self.carry {
      None => {
        let x = self.next_u64();
        self.carry = Some((x >> 32) as u32);
        x as u32
      },
      Some(x) => {
        self.carry = None;
        x
      }
    }
  }

  fn next_u64(&mut self) -> u64 {
    self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z: u64 = self.state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
  }
}

impl ::rand::SeedableRng<u64> for SplitMix64Rng {
  fn reseed(&mut self, seed: u64) {
    self.state = seed
  }

  fn from_seed(seed: u64) -> Self {
    SplitMix64Rng { state: seed, carry: None }
  }
}

/// Seed a SplitMix64Rng based on an OsRng.
///
/// # Panics
///
/// If OsRng is unavailable then this will panic. A safer option is `SplitMix64Rng::new()`
impl ::rand::SeedableRng<()> for SplitMix64Rng {
  fn reseed(&mut self, _: ()) {
    self.reseed(::rand::OsRng::new().unwrap().gen::<u64>())
  }

  fn from_seed(_: ()) -> Self {
    ::rand::OsRng::new().unwrap().gen::<Self>()
  }
}

impl ::rand::Rand for SplitMix64Rng {
  fn rand<R: ::rand::Rng>(rng: &mut R) -> SplitMix64Rng {
    SplitMix64Rng::from_seed(rng.gen::<u64>())
  }
}
