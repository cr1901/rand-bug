use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use rand::{Rng, SeedableRng};

fn random_ident() -> (u64, u64, [u8; 16], String) {
    static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let count: u64 = CALL_COUNT.fetch_add(1, Ordering::SeqCst) as u64;
    let mut seed: [u8; 16] = [0; 16];

    for (i, v) in seed.iter_mut().take(8).enumerate() {
        *v = ((secs >> (i * 8)) & 0xFF) as u8
    }

    for (i, v) in seed.iter_mut().skip(8).enumerate() {
        *v = ((count >> (i * 8)) & 0xFF) as u8
    }

    let mut rng = rand::rngs::SmallRng::from_seed(seed);
    (secs, count, seed, (0..16)
        .map(|i| {
            if i == 0 || rng.gen() {
                ('a' as u8 + rng.gen::<u8>() % 25) as char
            } else {
                ('0' as u8 + rng.gen::<u8>() % 10) as char
            }
        })
        .collect::<String>())
}

fn main() {
    println!("{:?}", random_ident());
    println!("{:?}", random_ident());
    println!("{:?}", random_ident());
}
