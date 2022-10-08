#![deny(unsafe_code)]
#![no_std]
#![cfg_attr(not(doc), no_main)]

use sisma as _; // global logger + panicking-behavior + memory layout

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{adc, pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
        // Configure ADC clocks
    // Default value is the slowest possible ADC clock: PCLK2 / 8. Meanwhile ADC
    // clock is configurable. So its frequency may be tweaked to meet certain
    // practical needs. User specified value is be approximated using supported
    // prescaler values 2/4/6/8.
    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);
    // defmt::println!("adc freq: {:?}", clocks.adcclk());


    // Setup ADC
    let mut adc1 = adc::Adc::adc1(dp.ADC1, clocks);

    #[cfg(any(feature = "stm32f103", feature = "connectivity"))]
    let mut adc2 = adc::Adc::adc2(p.ADC2, clocks); 

    // Setup GPIOB
    let mut gpiob = dp.GPIOB.split();
    

    // Configure pb0, pb1 as an analog input
    let mut ch0 = gpiob.pb0.into_analog(&mut gpiob.crl);

    #[cfg(any(feature = "stm32f103", feature = "connectivity"))]
    let mut ch1 = gpiob.pb1.into_analog(&mut gpiob.crl);



    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        defmt::println!("adc1: {}", data);

        #[cfg(any(feature = "stm32f103", feature = "connectivity"))]
        {
            let data1: u16 = adc2.read(&mut ch1).unwrap();
            defmt::println!("adc2: {}", data1);
        }
    }
}
