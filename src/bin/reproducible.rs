/*
Copyright (c) 2022 Todd Stellanova
LICENSE: See LICENSE file
*/
#![no_main]
#![no_std]

use analogulator as _; // global logger + panicking-behavior + memory layout

/// Example using a predicatable PRNG
///
///

use sensulator::{MeasureVal, Sensulator};
use defmt::println;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Latitude of Berkeley, California
const HOME_LAT:MeasureVal = 37.8716;
/// Absolute error of a typical GPS sensor (degrees)
const GPS_HORIZ_ABS_ERROR:MeasureVal = 2e-6;
/// Relative error of a typical GPS sensor (degrees)
const GPS_HORIZ_REL_ERROR:MeasureVal = 4.5e-5;

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("begin");

    // create a platform-independent predictable PRNG starting with a seed
    const HAY_SEED: [u8; 32] = [
        0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
        0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
        0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
        0xFE,0xED, 0xAB, 0xBA, 0xDE,0xAD, 0xBE, 0xEF,
    ];
    let mut my_rng = ChaCha8Rng::from_seed(HAY_SEED);

    let mut fake_gps_lat = Sensulator::new(HOME_LAT, GPS_HORIZ_ABS_ERROR, GPS_HORIZ_REL_ERROR, &mut my_rng);
    let mut old_peek_val = 0 as MeasureVal;
    for _in in 0..1000 {
        // read a noise-affected sensor value (requires a mutable sensulator reference)
        let cur_val: MeasureVal = fake_gps_lat.measure();
        println!("old: {} new: {}", old_peek_val, cur_val);

        // simply read the last measured value (may use an immutable reference)
        old_peek_val = fake_gps_lat.peek();
    }

    analogulator::exit()
}
