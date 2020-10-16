#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate heapless;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;
extern crate metro_core;

use cortex_m_semihosting::hprintln;
use rt::entry;

use metro_core::sequencer::sequencer;
use metro_core::musical::scale::Scale;

#[entry]
fn main() -> ! {
    let mut seq = sequencer::Sequencer::new();
    let scale = Scale::MinorBlues;
    loop {
        //Pretend our BPM is 1_000_000 loops
        for _ in 0..1_000_000 {
            //Read analog input
            let slider_0 = 0.0;
            let slider_1 = 0.1;
            let slider_2 = 0.2;
            let slider_3 = 0.4;
            let slider_4 = 0.5;
            let slider_5 = 0.7;
            let slider_6 = 0.8;
            let slider_7 = 1.0;

            //Update sequencer values from analog inputs
            seq.config().stage(0).unwrap().note = scale.quantize_float(slider_0);
            seq.config().stage(1).unwrap().note = scale.quantize_float(slider_1);
            seq.config().stage(2).unwrap().note = scale.quantize_float(slider_2);
            seq.config().stage(3).unwrap().note = scale.quantize_float(slider_3);
            seq.config().stage(4).unwrap().note = scale.quantize_float(slider_4);
            seq.config().stage(5).unwrap().note = scale.quantize_float(slider_5);
            seq.config().stage(6).unwrap().note = scale.quantize_float(slider_6);
            seq.config().stage(7).unwrap().note = scale.quantize_float(slider_7);

            //Get state of sequencer
            let state = seq.state();
            hprintln!("seq::state: {:?}", state).unwrap();
        }
        seq.step();
    }
}
