#![no_main]
#![no_std]

use sisma as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    sisma::exit()
}
