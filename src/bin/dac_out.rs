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
use p_hal::gpio::GpioExt;
use p_hal::rng::RngExt;
use p_hal::rcc::RccExt;
use p_hal::dac::{DacExt, DacOut};
use sensulator::{Sensulator};
use defmt::println;


#[cortex_m_rt::entry]
fn main() -> ! {
    println!("begin");

    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.require_pll48clk().freeze();
    let gpioa = dp.GPIOA.split();

    //PA4, PA5 support DAC
    let pin_raw = gpioa.pa4.into_analog();
    let mut pin_dac = dp.DAC.constrain(pin_raw);
    pin_dac.set_value(0);

    // create an unpredictable RNG using HAL-provided RNG
    let mut my_rng = dp.RNG.constrain(&clocks);

    // create some fake sensor values based on DAC 12-bit range
    const U16_MIDPOINT:f32 = (0x0FFF / 2) as f32;
    const U16_ABS_ERR:f32 = (0x0FFF/ 64)  as f32;
    const U16_REL_ERR:f32 = (0x0FFF / 16) as f32;

    let mut emulator = Sensulator::new(U16_MIDPOINT,U16_ABS_ERR, U16_REL_ERR, &mut my_rng);
    loop {
        // update the sensor reading and display (requires a mutable sensulator reference)
        let uval = emulator.measure() as u16;
        pin_dac.set_value(uval);
        let rval = pin_dac.get_value();
        println!("{} {}", uval, rval);
    }

    analogulator::exit()
}
