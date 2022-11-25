/*
Copyright (c) 2022 Todd Stellanova
LICENSE: See LICENSE file
*/
#![no_main]
#![no_std]

use analogulator as _; // global logger + panicking-behavior + memory layout

/// Example using a HAL-provided RNG
///
///
use stm32f4xx_hal as p_hal;
use p_hal::pac as pac;
use p_hal::rng::RngExt;
use p_hal::rcc::RccExt;
use sensulator::{MeasureVal, Sensulator};
// use rand::SeedableRng;
use defmt::println;

/// Latitude of Berkeley, California
const HOME_LAT:MeasureVal = 37.8716;
/// Absolute error of a typical GPS sensor (degrees)
const GPS_HORIZ_ABS_ERROR:MeasureVal = 2e-6;
/// Relative error of a typical GPS sensor (degrees)
const GPS_HORIZ_REL_ERROR:MeasureVal = 4.5e-5;

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("begin");

    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.require_pll48clk().freeze();
    // create an unpredictable RNG using HAL-provided RNG
    let mut my_rng = dp.RNG.constrain(&clocks);

    let mut fake_gps_lat = Sensulator::new(HOME_LAT, GPS_HORIZ_ABS_ERROR, GPS_HORIZ_REL_ERROR, &mut my_rng);
    loop {
        // update the sensor reading and display (requires a mutable sensulator reference)
        println!("new lat: {}", fake_gps_lat.measure());
        // simply display the last measured value (may use an immutable reference)
        println!("old lat: {}", fake_gps_lat.peek());
    }

    analogulator::exit()
}
