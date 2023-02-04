#![deny(unsafe_code)]
#![no_std]
#![cfg_attr(not(doc), no_main)]

use sisma as _; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use embedded_midi::{MidiMessage, MidiOut};
use nb::block;
use stm32f1xx_hal::{
    gpio::{gpioa::Parts, Input, Pin, PullDown, CRL},
    pac,
    prelude::*,
    serial::{Config, Serial},
    timer::Timer,
};

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

    let usart = Serial::usart1(
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        Config::default().baudrate(31250.bps()).parity_none(),
        clocks,
    );

    let (tx, _rx) = usart.split();

    let mut midi_out = MidiOut::new(tx);

    loop {
        //c4
        if inp_4.is_high() {
            defmt::println!("4 is pressed {:?}", inp_4.is_high());
            midi_out.write(&set_event(true, 60u8)).ok();
        } else if inp_4.is_low() {
            midi_out.write(&set_event(false, 60u8)).ok();
        }
        //d4
        if inp_5.is_high() {
            midi_out.write(&set_event(true, 62u8)).ok();
        } else if inp_5.is_low() {
            midi_out.write(&set_event(false, 62u8)).ok();
        }
        // if inp_6.is_high() {
        //     midi_out.write(&set_event(true, 64u8)).ok();
        // } else if inp_6.is_low() {
        //     midi_out.write(&set_event(false, 64u8)).ok();
        // }
    }
}

fn set_event(is_on: bool, note: u8) -> MidiMessage {
    if is_on {
        MidiMessage::NoteOn(0u8.into(), note.into(), 0x40u8.into())
    } else {
        MidiMessage::NoteOff(0u8.into(), note.into(), 0x40u8.into())
    }
}
