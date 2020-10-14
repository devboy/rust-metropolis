#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate heapless;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
use heapless::consts::*;
// use heapless::Vec;
// use hal::analog::adc::{Precision, SampleTime, VTemp};
// use hal::prelude::*;
// use hal::stm32;
use rt::entry;

use crate::metropolis::sequencer;
use crate::musical::note::Note;
use crate::musical::scale::Scale;

mod analog;
mod musical;
mod metropolis;

#[entry]
fn main() -> ! {
    let scale = Scale::MinorBlues;
    let mut seq = sequencer::Sequencer::<U8>::new();
    seq.config().stage(0).unwrap().skipped = false;
    seq.config().stage(1).unwrap().skipped = false;

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
