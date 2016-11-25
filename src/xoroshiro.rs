
pub struct Xoroshiro {
    state : [u64; 2],
    carry : Option<u32>
}

impl Xoroshiro {
    pub fn new(state: [u64; 2]) -> Self {
        Xoroshiro { state: state, carry: None }
    }
}

#[inline(always)]
fn rotate_left(x: u64, k: i32) -> u64 {
    (x << k) | (x >> (64 - k))
}

impl ::rand::Rng for Xoroshiro {
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
        let s0 : u64 = self.state[0];
        let mut s1 : u64 = self.state[1];
        let result : u64 = s0.wrapping_add(s1);

        s1 ^= s0;
        self.state[0] = rotate_left(s0, 55) ^ s1 ^ (s1 << 14); // a, b
        self.state[1] = rotate_left(s1, 36); // c

        return result;
    }
}
