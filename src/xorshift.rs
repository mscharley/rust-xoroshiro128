use {u64_from_sl, RngCore, SeedableRng};

#[derive(Debug, Clone, Copy)]
pub struct XorShift1024Rng {
  state: [u64; 16],
  pointer: usize,
}

impl XorShift1024Rng {
  #[cfg(feature = "rand")]
  /// Creates a new XorShift1024Rng instance which is randomly seeded.
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
  /// use xoroshiro128::XorShift1024Rng;
  ///
  /// let mut rng = XorShift1024Rng::new();
  /// let x: u32 = rng.gen();
  /// println!("{}", x);
  /// # }
  /// ```
  pub fn new() -> Self {
    use rand::Rng;
    let mut seed = <Self as SeedableRng>::Seed::default();
    for s in seed.0.iter_mut() {
      *s = rand::rngs::OsRng.gen();
    }
    Self::from_seed(seed)
  }

  pub fn from_seed_u64(seed: [u64; 16]) -> Self {
    assert!(
      &seed != &[0; 16],
      "Invalid seed: seed must not be 0."
    );
    XorShift1024Rng { state: seed, pointer: 0 }
  }
}

impl RngCore for XorShift1024Rng {
  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }

  fn next_u64(&mut self) -> u64 {
    let s0: u64 = self.state[self.pointer];
    self.pointer = (self.pointer + 1) & 15;
    let mut s1: u64 = self.state[self.pointer];
    s1 ^= s1 << 31;
    self.state[self.pointer] = s1 ^ s0 ^ (s1 >> 11) ^ (s0 >> 30);

    self.state[self.pointer].wrapping_mul(1181783497276652981u64)
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

pub struct XorShift1024Seed([u8; 16 * 8]);

impl Default for XorShift1024Seed {
  fn default() -> Self {
    Self([0; 16 * 8])
  }
}

impl core::convert::AsMut<[u8]> for XorShift1024Seed {
  fn as_mut(&mut self) -> &mut [u8] {
    &mut self.0[..]
  }
}

/// Seed a XorShift1024Rng with a given seed.
///
/// # Panics
///
/// XorShift1024Rng is undefined for the seed containing all zeros and will panic if this seed is
/// provided.
impl SeedableRng for XorShift1024Rng {
  type Seed = XorShift1024Seed;

  fn from_seed(seed: Self::Seed) -> Self {
    let mut state = [0; 16];
    for (offs, s) in state.iter_mut().enumerate() {
      *s = u64_from_sl(&seed.0[offs * 16..]);
    }
    Self::from_seed_u64(state)
  }
}
