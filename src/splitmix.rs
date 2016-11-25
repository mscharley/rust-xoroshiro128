use ::rand::SeedableRng;

pub struct SplitMixRng {
    state : u64,
    carry : Option<u32>
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

impl ::rand::Rand for SplitMixRng {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> SplitMixRng {
        SplitMixRng::from_seed(rng.gen::<u64>())
    }
}
