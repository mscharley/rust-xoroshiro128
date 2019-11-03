use {SeedableRng, RngCore, u64_from_sl};

#[derive(Debug, Clone, Copy)]
pub struct Xoroshiro128Rng {
  state: [u64; 2]
}

impl Xoroshiro128Rng {
  #[cfg(feature = "rand")]
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
  /// # extern crate rand; extern crate xoroshiro128; fn main() {
  /// use rand::Rng;
  /// use xoroshiro128::Xoroshiro128Rng;
  ///
  /// let mut rng = Xoroshiro128Rng::new();
  /// let x: u32 = rng.gen();
  /// println!("{}", x);
  /// # }
  /// ```
  pub fn new() -> Self {
    use rand::Rng;
    let seed = rand::rngs::OsRng.gen::<<Self as SeedableRng>::Seed>();
    Self::from_seed(seed)
  }

  /// Creates a new `Xoroshiro128Rng` instance which is not seeded.
  ///
  /// The initial values of this Rng are constants, so all generators created by this function
  /// will yield the same stream of random numbers. It is highly recommended that this is created
  /// through `SeedableRng` instead of this function.
  pub fn new_unseeded() -> Self {
    Xoroshiro128Rng::from_seed_u64([
      0x193a6754a8a7d469,
      0x97830e05113ba7bb
    ])
  }

  pub fn from_seed_u64(seed: [u64; 2]) -> Self {
    assert!(seed != [0; 2], "Invalid seed: seed must not be 0.");
    Xoroshiro128Rng { state: seed }
  }
}

impl RngCore for Xoroshiro128Rng {
  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }

  fn next_u64(&mut self) -> u64 {
    let s0: u64 = self.state[0];
    let mut s1: u64 = self.state[1];
    let result: u64 = s0.wrapping_add(s1);

    s1 ^= s0;
    self.state[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
    self.state[1] = s1.rotate_left(36); // c

    result
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

/// Seed a `Xoroshiro128Rng` with a given seed.
///
/// # Panics
///
/// `Xoroshiro128Rng` is undefined for the seed `[0, 0]` and will panic if this seed is provided.
impl SeedableRng for Xoroshiro128Rng {
  type Seed = [u8; 16];

  fn from_seed(seed: [u8; 16]) -> Self {
    Self::from_seed_u64([u64_from_sl(&seed[..]), u64_from_sl(&seed[8..])])
  }
}