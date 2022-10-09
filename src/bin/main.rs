#![deny(unsafe_code)]
#![no_std]
#![cfg_attr(not(doc), no_main)]

use sisma as _; // global logger + panicking-behavior + memory layout

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{adc, pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // device specific peripherals from PAC
    let dp = pac::Peripherals::take().unwrap();
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
// `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);


    // Setup GPIOB
    let mut gpioa = dp.GPIOA.split();
    

    // Configure pa4, pb1 as pull down input
    let mut in_4 = gpioa.pa4.as_pull_down_input(&mut gpioa.crl, f);// what is f?





    loop {

    }
}
