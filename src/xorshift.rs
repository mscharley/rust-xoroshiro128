use std::io;
use {Rng, SeedableRng, Rand};

const ZERO_SEED: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[derive(Debug, Clone, Copy)]
pub struct XorShift1024Rng {
  state: [u64; 16],
  pointer: usize
}

impl XorShift1024Rng {
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
  /// use xoroshiro128::{Rng, XorShift1024Rng};
  ///
  /// let rng = XorShift1024Rng::new();
  /// let x: u32 = rng.unwrap().gen();
  /// println!("{}", x);
  /// ```
  pub fn new() -> io::Result<Self> {
    ::rand::OsRng::new().map(|mut x: ::rand::OsRng| x.gen::<Self>())
  }
}

impl Rng for XorShift1024Rng {
  fn next_u32(&mut self) -> u32 {
    self.next_u64() as u32
  }

  fn next_u64(&mut self) -> u64 {
    let s0 : u64 = self.state[self.pointer];
    self.pointer = (self.pointer + 1) & 15;
    let mut s1 : u64 = self.state[self.pointer];
    s1 ^= s1 << 31;
    self.state[self.pointer] = s1 ^ s0 ^ (s1 >> 11) ^ (s0 >> 30);

    self.state[self.pointer].wrapping_mul(1181783497276652981u64)
  }
}

/// Seed a XorShift1024Rng with a given seed.
///
/// # Panics
///
/// XorShift1024Rng is undefined for the seed containing all zeros and will panic if this seed is
/// provided.
impl SeedableRng<[u64; 16]> for XorShift1024Rng {
  fn reseed(&mut self, seed: [u64; 16]) {
    assert!(seed != ZERO_SEED, "Invalid seed: seed must not be 0.");
    self.state = seed
  }

  fn from_seed(seed: [u64; 16]) -> Self {
    assert!(seed != ZERO_SEED, "Invalid seed: seed must not be 0.");
    XorShift1024Rng { state: seed, pointer: 0 }
  }
}

#[inline(always)]
fn splitmix_seed(seed: u64) -> [u64; 16] {
  ::SplitMix64Rng::from_seed(seed).gen()
}

impl SeedableRng<u64> for XorShift1024Rng {
  fn reseed(&mut self, seed: u64) {
    self.reseed(splitmix_seed(seed))
  }

  fn from_seed(seed: u64) -> Self {
    XorShift1024Rng::from_seed(splitmix_seed(seed))
  }
}


impl Rand for XorShift1024Rng {
  fn rand<R: Rng>(rng: &mut R) -> XorShift1024Rng {
    let mut seed: [u64; 16] = rng.gen();
    while seed == ZERO_SEED {
      seed = rng.gen();
    }

    XorShift1024Rng::from_seed(seed)
  }
}
