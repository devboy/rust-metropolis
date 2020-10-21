#![deny(warnings)]
// #![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate heapless;
extern crate metro_core;
extern crate stm32g0xx_hal as hal;

use analog_multiplexer::Multiplexer;
use cortex_m_semihosting::hprintln;
// use embedded_hal::digital::v2::StatefulOutputPin;
use hal::analog::adc::{AdcExt, Precision, SampleTime};
use hal::analog::dac::{DacExt, DacOut};
use hal::delay::DelayExt;
use hal::gpio::{GpioExt, Speed};
use hal::hal::adc::OneShot;
use hal::hal::timer::CountDown;
use hal::prelude::OutputPin;
use hal::rcc::{Config, RccExt};
use hal::stm32;
use hal::time::U32Ext;
use hal::timer::TimerExt;
use micromath::F32Ext;
// extern crate nb;
// extern crate panic_halt;
use panic_semihosting as _;
use rt::entry;
use stm32g0::stm32g071::TIM17;

use metro_core::musical::gate::Gate;
use metro_core::musical::scale::Scale;
use metro_core::sequencer::sequencer;
use metro_core::sequencer::sequencer::GateMode;
use metro_core::sequencer::stage_mode::StageMode;

const N: usize = 8;
const BPM: u32 = 128;
const STEP_PM: u32 = BPM * 4;
const STEP_DUR: u16 = ((60_f32 / STEP_PM as f32) * 1_000_f32) as u16;
const GATE_DUR: u16 = STEP_DUR / 2;

#[entry]
unsafe fn main() -> ! {
    hprintln!("N: {}", N).unwrap();
    hprintln!("BPM: {}", BPM).unwrap();
    hprintln!("STEP_PM: {}", STEP_PM).unwrap();
    hprintln!("STEP_DUR: {}ms", STEP_DUR).unwrap();
    hprintln!("GATE_DUR: {}ms", GATE_DUR).unwrap();

    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain().freeze(Config::pll());
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
    let mut gate_led = gpiob.pb4.into_push_pull_output().set_speed(Speed::VeryHigh);

    // Outputs
    let mut gate = gpiob.pb5.into_push_pull_output();
    let dac0 = dp.DAC.constrain(gpioa.pa4, &mut rcc);
    let mut pitch = dac0.calibrate_buffer(&mut delay).enable();

    let mut seq = sequencer::Sequencer::new();
    seq.config().set_stage_mode(StageMode::PingPong);
    seq.config().set_gate_time_us((GATE_DUR as u32 * 1000) as u32);
    let scale = Scale::Chromatic;

    let mut timer = dp.TIM17.timer(&mut rcc);
    timer.start(1000.ms());
    (*TIM17::ptr()).psc.modify(|_, w| w.psc().bits(64000 - 1));

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
            stage.gate_mode = GateMode::Repeat;
            stage.pulse_count = F32Ext::round(pulse_counts[s] * N as f32) as u8;
            stage.pulse_count = 1u8;
        }

        seq.config().set_rnd_seed(tim17_cnt() as u32); // TODO: Find a better seed

        //Trigger Step
        if tim17_cnt() >= STEP_DUR {
            seq.step();
            tim17_rst();
        }

        //Get state of sequencer
        let state = seq.state(tim17_cnt() as u32 * 1000_u32); // TODO: Refactor to ms
        pitch.set_value((state.note.voltage() * (4_095_f32 / 3.3)) as u16);
        mux_out.set_channel(state.pos.stage);
        match state.gate {
            Gate::Open => {
                gate.set_high().unwrap();
                gate_led.set_high().unwrap();
            }
            Gate::Closed => {
                gate.set_low().unwrap();
                gate_led.set_low().unwrap();
            }
        }
    }
}

unsafe fn tim17_rst() {
    (*TIM17::ptr()).cnt.modify(|r, w|
        w.cnt().bits(r.cnt().bits().saturating_sub(STEP_DUR)))
}

unsafe fn tim17_cnt() -> u16 {
    (*TIM17::ptr()).cnt.read().cnt().bits()
}
