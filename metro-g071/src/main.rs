#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate heapless;
extern crate metro_core;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;

use analog_multiplexer::Multiplexer;
use hal::analog::adc::{AdcExt, Precision, SampleTime};
use hal::gpio::GpioExt;
use hal::hal::adc::OneShot;
use hal::rcc::RccExt;
use hal::stm32;
use hal::time::MicroSecond;
use hal::timer::stopwatch::StopwatchExt;
use metro_core::musical::scale::Scale;
use metro_core::sequencer::sequencer;
use rt::entry;
use metro_core::sequencer::sequencer::GateMode;
use micromath::F32Ext;
use hal::analog::dac::{DacExt, DacOut};
use metro_core::musical::gate::Gate;
use hal::prelude::OutputPin;
use hal::delay::DelayExt;

const N: usize = 8;
const BPM: u32 = 128;
const BD: MicroSecond = MicroSecond(((60_f32 / BPM as f32) * 1000_f32) as u32);
const GD: u32 = BD.0 / 2;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let mut adc = dp.ADC.constrain(&mut rcc);
    adc.set_sample_time(SampleTime::T_80);
    adc.set_precision(Precision::B_12);
    let mut delay = cp.SYST.delay(&mut rcc);
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Multiplexer Inputs
    let mux_in_0 = gpiob.pb12.into_push_pull_output();
    let mux_in_1 = gpiob.pb13.into_push_pull_output();
    let mux_in_2 = gpiob.pb14.into_push_pull_output();
    let mux_in_en = gpiob.pb15.into_push_pull_output();
    let mut mux_in = Multiplexer::new((mux_in_0, mux_in_1, mux_in_2, mux_in_en));
    let mut a_pitch = gpioa.pa0.into_analog();
    let mut a_pulse_count = gpioa.pa1.into_analog();
    let mut a_gate_mode = gpioa.pa2.into_analog();

    // Multiplexer Outputs
    let mux_out_0 = gpiob.pb0.into_push_pull_output();
    let mux_out_1 = gpiob.pb1.into_push_pull_output();
    let mux_out_2 = gpiob.pb2.into_push_pull_output();
    let mux_out_en = gpiob.pb3.into_push_pull_output();
    let mut mux_out = Multiplexer::new((mux_out_0, mux_out_1, mux_out_2, mux_out_en));
    let mut gate_led = gpiob.pb4.into_push_pull_output();

    // Outputs
    let mut gate = gpiob.pb5.into_push_pull_output();
    let dac0 = dp.DAC.constrain(gpioa.pa4, &mut rcc);
    let mut pitch = dac0.calibrate_buffer(&mut delay).enable();

    let stopwatch = dp.TIM3.stopwatch(&mut rcc);

    let mut seq = sequencer::Sequencer::new();
    seq.set_gate_time_ms(GD);
    let scale = Scale::MinorBlues;
    let mut last_beat = stopwatch.now();

    let mut pitches = [0_f32; N];
    let mut pulse_counts = [0_f32; N];
    let mut gate_modes = [0_f32; N];

    loop {
        // Read analog values from mux
        for ch in 0..N {
            mux_in.set_channel(ch as u8);
            let pitch: u32 = adc.read(&mut a_pitch).unwrap();
            pitches[ch] = pitch.saturating_sub(32) as f32 / 4_096_f32;
            let pulse_count: u32 = adc.read(&mut a_pulse_count).unwrap();
            pulse_counts[ch] = pulse_count.saturating_sub(32) as f32 / 4_096_f32;
            let gate_mode: u32 = adc.read(&mut a_gate_mode).unwrap();
            gate_modes[ch] = gate_mode.saturating_sub(32) as f32 / 4_096_f32;
        }

        // Configure sequencer
        for s in 0..N {
            let stage = seq.config().stage(s).unwrap();
            stage.note = scale.quantize_float(pitches[s]);
            stage.gate_mode = GateMode::from_float(gate_modes[s]);
            stage.pulse_count = F32Ext::round(pulse_counts[s] * N as f32) as u8;
        }

        //Trigger BPM
        if stopwatch.elapsed(last_beat) > BD {
            seq.step();
            last_beat = stopwatch.now();
        }

        //Get state of sequencer
        let state = seq.state(stopwatch.elapsed(last_beat).0);
        pitch.set_value((state.note.voltage() * (4_095_f32 / 3.3)) as u16);
        match state.gate {
            Gate::Open => {
                gate.set_high().unwrap();
                gate_led.set_high().unwrap();
            },
            Gate::Closed => {
                gate.set_low().unwrap();
                gate_led.set_low().unwrap();
            },
        }
        mux_out.set_channel(state.pos.stage);
    }
}
