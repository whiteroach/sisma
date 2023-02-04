#![no_std]
#![cfg_attr(not(doc), no_main)]
use stm32f1xx_hal::gpio::{gpioa::Parts, Input, Pin, PullDown, CRL};
pub struct WhiteKeys {
    c: Pin<Input<PullDown>, CRL, 'A', 0>,
    d: Pin<Input<PullDown>, CRL, 'A', 1>,
    e: Pin<Input<PullDown>, CRL, 'A', 2>,
    f: Pin<Input<PullDown>, CRL, 'A', 3>,
    g: Pin<Input<PullDown>, CRL, 'A', 4>,
    a: Pin<Input<PullDown>, CRL, 'A', 5>,
    b: Pin<Input<PullDown>, CRL, 'A', 6>,
    c_min: Pin<Input<PullDown>, CRL, 'A', 7>,
}

impl WhiteKeys {
    fn new(mut parts: Parts) -> Self {
        WhiteKeys {
            c: parts.pa0.into_pull_down_input(&mut parts.crl),
            d: parts.pa1.into_pull_down_input(&mut parts.crl),
            e: parts.pa2.into_pull_down_input(&mut parts.crl),
            f: parts.pa3.into_pull_down_input(&mut parts.crl),
            g: parts.pa4.into_pull_down_input(&mut parts.crl),
            a: parts.pa5.into_pull_down_input(&mut parts.crl),
            b: parts.pa6.into_pull_down_input(&mut parts.crl),
            c_min: parts.pa7.into_pull_down_input(&mut parts.crl),
        }
    }
}
