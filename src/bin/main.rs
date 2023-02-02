#![deny(unsafe_code)]
#![no_std]
#![cfg_attr(not(doc), no_main)]

use sisma as _; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use embedded_midi::{MidiMessage, MidiOut};
use nb::block;
use stm32f1xx_hal::{
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

    // Configure the syst timer to trigger an update every 50ms
    // let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    // timer.start(1.Hz()).unwrap(); //50 ms

    // let event_on = MidiMessage::NoteOn(0u8.into(), 40u8.into(), 0x40u8.into());
    // let event_off = MidiMessage::NoteOff(0u8.into(), 40u8.into(), 0x40u8.into());
    // let p = MidiMessage::NoteOn(0u8.into(), 72u8.into(), 0x40u8.into());
    // let p_l= MidiMessage::NoteOff(0u8.into(), 72u8.into(), 0x40u8.into());
    // let on_four = event_on.clone();
    // let off_four = event_off.clone();
    loop {
        if inp_4.is_high(){
            defmt::println!("4 is pressed {:?}",inp_4.is_high());
            if led.is_set_high() {
                led.set_low();// it lit!

                
                midi_out.write(&set_event(true,40u8 )).ok();
            }
        } else if inp_4.is_low(){
            // // defmt::println!("4 is low low")
            if led.is_set_low() {
                    led.set_high();// darkness
                    midi_out.write(&set_event(false,40u8 )).ok();
                    
                }
                // midi_out.write(&set_event(false,40u8 )).ok();
            // midi_out.write(&set_event(true,40u8 )).ok();
        } 
        // if inp_5.is_high() {
        //     if led.is_set_high() {
        //         led.set_low();

        //         midi_out.write(&set_event(true,72u8 )).ok();
                
        //     }
        // } else if inp_5.is_low() {
        //     if led.is_set_low() {
        //         led.set_high();
        //         midi_out.write(&set_event(false,72u8 )).ok();
                
        //     }
        // }
        // if inp_5.is_high() {
        //     // block!(timer.wait()).unwrap();
        //     if led.is_set_high() {
        //         led.set_low();

        //         midi_out.write(&MidiMessage::NoteOn(0u8.into(), 50u8.into(), 0x40u8.into())).ok();
        //     }
        //     // defmt::println!("5 is high")
        // } else if inp_5.is_low() {
        //     // block!(timer.wait()).unwrap();
        //     if led.is_set_low() {

        //         led.set_high();
        //         midi_out.write(&MidiMessage::NoteOff(0u8.into(), 50u8.into(), 0x40u8.into())).ok();
        //     }

        // } else if inp_4.is_high() {
        //     defmt::println!("4 is high");
        //     if led.is_set_high() {
        //         led.set_low();

        //         midi_out.write(&MidiMessage::NoteOn(0u8.into(), 50u8.into(), 0x40u8.into())).ok();
        //     }
        // } else if inp_4.is_low() {
        //     if led.is_set_low() {

        //         led.set_high();
        //         midi_out.write(&MidiMessage::NoteOff(0u8.into(), 50u8.into(), 0x40u8.into())).ok();
        //     }
        // }
        // let event = MidiMessage::NoteOn(0u8.into(), 50u8.into(), 0x40u8.into());
        //     midi_out.write(&event).ok();
        //     let event = MidiMessage::NoteOff(0u8.into(), 50u8.into(), 0x40u8.into());
        //     midi_out.write(&event).ok();
        // if inp_4.is_high() {
        // } else if inp_5.is_high() {
        //     midi_out.write(&event).ok();
        // } else if inp_4.is_low(){
        //     let event = MidiMessage::NoteOff(0u8.into(), 50u8.into(), 0x40u8.into());
        //     midi_out.write(&event).ok();
        // } else if inp_5.is_low(){
        //     let event = MidiMessage::NoteOff(0u8.into(), 50u8.into(), 0x40u8.into());
        //     midi_out.write(&event).ok();
        // }

        // if inp_4.is_high() {
        //     defmt::println!("4 is high");
        //     midi_out.write(&on_four).ok();
        // } else if inp_4.is_low() {
        //     defmt::println!("4 is low");
        //     midi_out.write(&off_four).ok();
        // }
    }
}

fn set_event(is_on:bool,note:u8) -> MidiMessage {
    if is_on {
        MidiMessage::NoteOn(0u8.into(), note.into(), 0x40u8.into())
    } else {
        MidiMessage::NoteOff(0u8.into(), note.into(),0x40u8.into())

    }
}