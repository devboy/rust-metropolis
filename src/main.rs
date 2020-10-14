#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
// use hal::analog::adc::{Precision, SampleTime, VTemp};
// use hal::prelude::*;
// use hal::stm32;
use rt::entry;

mod analog;
mod musical;
mod metropolis;

// use crate::analog::note as _;
use crate::musical::note::{Note};
use crate::musical::scale::Scale;
use crate::metropolis::sequencer;

#[entry]
fn main() -> ! {
    let scale = Scale::Bassline;
    let _sequencer = sequencer::Sequencer::new(sequencer::Config::new());
    hprintln!("convert C: {:?}", scale.quantize(Note::C)).unwrap();
    hprintln!("convert D: {:?}", scale.quantize(Note::D)).unwrap();
    hprintln!("convert E: {:?}", scale.quantize(Note::E)).unwrap();
    hprintln!("convert F: {:?}", scale.quantize(Note::F)).unwrap();
    hprintln!("convert G: {:?}", scale.quantize(Note::G)).unwrap();
    hprintln!("convert A: {:?}", scale.quantize(Note::A)).unwrap();
    hprintln!("convert B: {:?}", scale.quantize(Note::B)).unwrap();
    // let seq = Sequencer::new(8);
    // for _ in 0..128 {
    //     let state = seq.tick();
    //     println!("state {}", state);
    // }
    loop {}
}
