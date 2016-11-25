
use ::rand::Rng;
use super::Xoroshiro;

#[test]
fn it_works_u32() {
    let mut rng = Xoroshiro::new([0u64, 0u64]);
    let x: u32 = rng.gen();
    let y: u32 = rng.gen();
    assert_eq!(x, 10);
    assert_eq!(y, 0);
}

#[test]
fn it_works_u64() {
    let mut rng = Xoroshiro::new([0u64, 0u64]);
    let x: u64 = rng.gen();
    assert_eq!(x, 10);
}
