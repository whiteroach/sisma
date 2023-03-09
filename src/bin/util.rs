#![no_std]
#![cfg_attr(not(doc), no_main)]
use embedded_midi::MidiOut;
use stm32f1xx_hal::gpio::{gpioa::Parts, Input, Pin, PullDown, CRL,CRH};
pub struct Keys {
    pub c: Pin<Input<PullDown>, CRL, 'A', 0>,
    pub d_b: Pin<Input<PullDown>, CRH, 'A', 8>,
    pub d: Pin<Input<PullDown>, CRL, 'A', 1>,
    pub e_b: Pin<Input<PullDown>, CRH, 'A', 9>,
    pub e: Pin<Input<PullDown>, CRL, 'A', 2>,
    pub f: Pin<Input<PullDown>, CRL, 'A', 3>,
    pub g_b: Pin<Input<PullDown>, CRH, 'A', 10>,
    pub g: Pin<Input<PullDown>, CRL, 'A', 4>,
    pub a_b: Pin<Input<PullDown>, CRH, 'A', 11>
    // a: Pin<Input<PullDown>, CRL, 'A', 5>,
    // b: Pin<Input<PullDown>, CRL, 'A', 6>,
    // c_min: Pin<Input<PullDown>, CRL, 'A', 7>,
}

impl Keys {
   pub fn new(mut parts: Parts) -> Self {
        Keys {
            c: parts.pa0.into_pull_down_input(&mut parts.crl),
            d_b: parts.pa8.into_pull_down_input(&mut parts.crh),
            d: parts.pa1.into_pull_down_input(&mut parts.crl),
            e_b: parts.pa9.into_pull_down_input(&mut parts.crh),
            e: parts.pa2.into_pull_down_input(&mut parts.crl),
            f: parts.pa3.into_pull_down_input(&mut parts.crl),
            g_b: parts.pa10.into_pull_down_input(&mut parts.crh),
            g: parts.pa4.into_pull_down_input(&mut parts.crl),
            a_b:parts.pa11.into_pull_down_input(&mut parts.crh)
            // f: parts.pa3.into_pull_down_input(&mut parts.crl),
            // g: parts.pa4.into_pull_down_input(&mut parts.crl),
            // a: parts.pa5.into_pull_down_input(&mut parts.crl),
            // b: parts.pa6.into_pull_down_input(&mut parts.crl),
            // c_min: parts.pa7.into_pull_down_input(&mut parts.crl),
        }
    }
}
