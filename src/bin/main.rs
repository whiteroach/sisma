#![deny(unsafe_code)]
#![no_std]
#![cfg_attr(not(doc), no_main)]

use sisma as _; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use embedded_midi::{MidiMessage, MidiOut};
use nb::block;
use stm32f1xx_hal::{
    gpio::{gpioa::Parts, Input, Pin, PullDown, CRL,CRH, gpiob},
    pac,
    prelude::*,
    serial::{Config, Serial},
    timer::Timer,
};
mod util;
use util::Keys;

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
    // Setup GPIOB
    let mut gpiob = dp.GPIOB.split();
    //Setup GPIOC
    let mut gpioc = dp.GPIOC.split();


    // Configure pa4 as pull down input
    // let mut inp_4 = gpioa.pa4.into_pull_down_input(&mut gpioa.crl);
    //Configure pa5 as pull down
    // let mut inp_5 = gpioa.pa5.into_pull_down_input(&mut gpioa.crl);
    //Configure embed led
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut w_keys = Keys::new( gpioa);

    // USART1 on pins B6 and B7
    let pin_tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let pin_rx = gpiob.pb7;// Can I free this pin?
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
        //C4
        if w_keys.c.is_high() {
            midi_out.write(&set_event(true, 60u8)).ok();
        } else if w_keys.c.is_low() {
            midi_out.write(&set_event(false, 60u8)).ok();
        }
        //Db
        if w_keys.d_b.is_high() {
            midi_out.write(&set_event(true, 61u8)).ok();
        } else if w_keys.d_b.is_low() {
            midi_out.write(&set_event(false, 61u8)).ok();
        }
        //D4
        if w_keys.d.is_high() {
            midi_out.write(&set_event(true, 62u8)).ok();
        } else if w_keys.d.is_low() {
            midi_out.write(&set_event(false, 62u8)).ok();
        }
        //Eb4
        if w_keys.e_b.is_high() {
            midi_out.write(&set_event(true, 63u8)).ok();
        } else if w_keys.e_b.is_low() {
            midi_out.write(&set_event(false, 63u8)).ok();
        }
        //E4
        if w_keys.e.is_high() {
            midi_out.write(&set_event(true, 64u8)).ok();
        } else if w_keys.e.is_low() {
            midi_out.write(&set_event(false, 64u8)).ok();
        }
        //F4
        if w_keys.f.is_high() {
            midi_out.write(&set_event(true, 65u8)).ok();
        } else if w_keys.f.is_low() {
            midi_out.write(&set_event(false, 65u8)).ok();
        }
        //Gb4
        if w_keys.g_b.is_high() {
            midi_out.write(&set_event(true, 66u8)).ok();
        } else if w_keys.g_b.is_low() {
            midi_out.write(&set_event(false, 66u8)).ok();
        }
        //G4
        if w_keys.g.is_high() {
            midi_out.write(&set_event(true, 67u8)).ok();
        } else if w_keys.g.is_low() {
            midi_out.write(&set_event(false, 67u8)).ok();
        }
    }
}

fn set_event(is_on: bool, note: u8) -> MidiMessage {
    if is_on {
        MidiMessage::NoteOn(0u8.into(), note.into(), 0x40u8.into())
    } else {
        MidiMessage::NoteOff(0u8.into(), note.into(), 0x40u8.into())
    }
}
