use std::io;
use ::rand::{Rng, SeedableRng};

pub struct SplitMixRng {
    state : u64,
    carry : Option<u32>
}

impl SplitMixRng {
    /// Creates a new SplitMixRng instance which is randomly seeded.
    ///
    /// # Errors
    ///
    /// When created this way the initial values of this Rng are seeded using an OsRng. If this
    /// fails then this method will pass through the error from OsRng.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # extern crate rand;
    /// # extern crate xoroshiro128;
    /// use xoroshiro128::SplitMixRng;
    /// use rand::Rng;
    ///
    /// # fn main() {
    /// let rng = SplitMixRng::new();
    /// let x: u32 = rng.unwrap().gen();
    /// println!("{}", x);
    /// # }
    /// ```
    pub fn new() -> io::Result<Self> {
        ::rand::OsRng::new().map(|mut x: ::rand::OsRng| x.gen::<Self>())
    }
}

impl ::rand::Rng for SplitMixRng {
    fn next_u32(&mut self) -> u32 {
        match self.carry {
            None => {
                let x = self.next_u64();
                self.carry = Some((x >> 32) as u32);
                x as u32
            },
            Some(x) => { self.carry = None; x }
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z : u64 = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

impl ::rand::SeedableRng<u64> for SplitMixRng {
    fn reseed(&mut self, seed: u64) {
        self.state = seed
    }

    fn from_seed(seed: u64) -> Self {
        SplitMixRng { state: seed, carry: None }
    }
}

/// Seed a SplitMixRng based on an OsRng.
///
/// # Panics
///
/// If OsRng is unavailable then this will panic. A safer option is `SplitMixRng::new()`
impl ::rand::SeedableRng<()> for SplitMixRng {
    fn reseed(&mut self, _: ()) {
        self.reseed(::rand::OsRng::new().unwrap().gen::<u64>())
    }

    fn from_seed(_: ()) -> Self {
        ::rand::OsRng::new().unwrap().gen::<Self>()
    }
}

impl ::rand::Rand for SplitMixRng {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> SplitMixRng {
        SplitMixRng::from_seed(rng.gen::<u64>())
    }
}
