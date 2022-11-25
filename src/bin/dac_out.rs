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
use sensulator::{MeasureVal, Sensulator};
use defmt::println;


// create some fake sensor values based on a 12-bit DAC range
const MOCK_SENSOR_CTR_VAL:MeasureVal = (0x0FFF / 2) as MeasureVal;
const MOCK_SENSOR_ABS_ERR:MeasureVal = (0x0FFF/ 64) as MeasureVal;
const MOCK_SENSOR_REL_ERR:MeasureVal = (0x0FFF / 16) as MeasureVal;

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

    //example: create an unpredictable RNG using HAL-provided RNG
    let mut my_rng = dp.RNG.constrain(&clocks);

    let mut sensor = Sensulator::new(MOCK_SENSOR_CTR_VAL, MOCK_SENSOR_ABS_ERR, MOCK_SENSOR_REL_ERR, &mut my_rng);
    for _n in 0..1000 {
        // update the sensor reading and display (requires a mutable sensulator reference)
        let uval = sensor.measure() as u16;
        pin_dac.set_value(uval);
        let rval = pin_dac.get_value();
        println!("{} {}", uval, rval);
    }

    analogulator::exit()
}
