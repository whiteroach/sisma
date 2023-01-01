#![deny(unsafe_code)]
#![no_std]
#![cfg_attr(not(doc), no_main)]

use sisma as _; // global logger + panicking-behavior + memory layout

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer, serial::{Config, Serial}};

#[entry]
fn main() -> ! {
    // core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // device specific peripherals from PAC
    let dp = pac::Peripherals::take().unwrap();
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Setup GPIOA
    let mut gpioa = dp.GPIOA.split();
    //Setup GPIOC
    let mut gpioc = dp.GPIOC.split();

    // Configure pa4 as pull down input
    let mut inp_4 = gpioa.pa4.into_pull_down_input(&mut gpioa.crl);
    //Configure pa5 as pull down
    let mut inp_5 = gpioa.pa5.into_pull_down_input(&mut gpioa.crl);
    //Configure embed led
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // USART1 on Pins A9 and A10
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10;

    let midi_serial = Serial::usart1(
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(31250.bps())
            .wordlength_8bits()
            .parity_none(),
        clocks,
     );
    // let serial = midi_serial.with_u16_data();
    let (mut tx, mut rx) = midi_serial.split();
    // Configure the syst timer to trigger an update every 50ms
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap(); //50 ms

    loop {
        if inp_4.is_high() {
            defmt::println!("4 is high")
        } else if inp_4.is_low() {
            defmt::println!("4 is low low")
        }
        if inp_5.is_high() {
            // block!(timer.wait()).unwrap();
            if led.is_set_high() {
                led.set_low();
            }
            defmt::println!("5 is high")
        } else {
            // block!(timer.wait()).unwrap();
            if led.is_set_low() {
                led.set_high();
            }

            defmt::println!("5 is low")
        }
    }
}
