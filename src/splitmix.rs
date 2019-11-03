use {RngCore, SeedableRng};

#[derive(Debug, Clone, Copy)]
pub struct SplitMix64Rng {
  state: u64,
}

impl SplitMix64Rng {
  #[cfg(feature = "rand")]
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
  /// # extern crate rand; extern crate xoroshiro128; fn main() {
  /// use rand::Rng;
  /// use xoroshiro128::SplitMix64Rng;
  ///
  /// let mut rng = SplitMix64Rng::new();
  /// let x: u32 = rng.gen();
  /// println!("{}", x);
  /// # }
  /// ```
  pub fn new() -> Self {
    use rand::Rng;
    let seed = rand::rngs::OsRng.gen::<<Self as SeedableRng>::Seed>();
    Self::from_seed(seed)
  }

  pub fn from_seed_u64(seed: u64) -> Self {
    SplitMix64Rng { state: seed }
  }
}

impl RngCore for SplitMix64Rng {
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

  fn fill_bytes(&mut self, dest: &mut [u8]) {
    let mut ctr = 0;
    let mut v = 0;
    for d in dest.iter_mut() {
      if ctr == 0 {
        v = self.next_u64();
        ctr = 7;
      }
      *d = v as u8;
      v >>= 8;
      ctr -= 1;
    }
  }
  fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
    self.fill_bytes(dest);
    Ok(())
  }
}

impl SeedableRng for SplitMix64Rng {
  type Seed = [u8; 8];
  fn from_seed(seed: [u8; 8]) -> Self {
    Self::from_seed_u64(u64::from_le_bytes(seed))
  }
}
