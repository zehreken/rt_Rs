use ringbuf::HeapProducer;

use crate::{
    audio::{lfo, oscillator_type::OscillatorType, songs},
    basics::core::clamp,
};

use super::{lfo::LFO, vco::VCO};

pub struct Sequencer {
    pub is_running: bool,
    vco: VCO,
    lfo: LFO,
    beat_index: u32,
    length: u8,
    freq: f32,
    sequence: Vec<f32>,
    tick_period: f32,
    beat_period: f32,
    is_beat: bool,
    ramp: f32,
    tick: u32,
    signal: f32,
}

impl Sequencer {
    pub fn new(
        bpm: u16,
        sample_rate: u32,
        channel_count: u32,
        sequence: Vec<f32>, // song
    ) -> Self {
        let tick_period = (sample_rate * 60) as f32 / bpm as f32;
        let beat_duration = tick_period / 3.0;
        println!("Sequencer: {bpm}, {sample_rate}, {channel_count}, {tick_period}");
        let mut vco = VCO::new(sample_rate as f32);
        vco.set_wave_type(kopek::oscillator::WaveType::Sine);
        let mut lfo = LFO::new(sample_rate as f32);
        lfo.set_frequency(10.0);

        Self {
            is_running: false,
            vco,
            lfo,
            beat_index: 0,
            length: sequence.len() as u8,
            freq: sequence[0],
            sequence,
            tick_period,
            beat_period: beat_duration,
            is_beat: false,
            ramp: 0.0,
            tick: 0,
            signal: 0.0,
        }
    }

    pub fn update(&mut self, elapsed_samples: u32) -> f32 {
        let remainder = elapsed_samples % self.tick_period as u32;
        self.is_beat = remainder > 0 && remainder < self.beat_period as u32;
        self.beat_index = elapsed_samples / self.tick_period as u32;
        let step_index = (self.beat_index % self.length as u32) as usize;
        const TEMP_OCTAVE: u8 = 2u8.pow(4);
        let freq_diff: f32 = if step_index == 0 {
            0.0
        } else {
            // Reach next freq in 50 samples
            self.sequence[step_index] - self.sequence[step_index - 1] / 50.0
        };

        let mut value = 0.0;
        // Ramp between steps
        // if self.freq != self.freqs[step_index] {
        //     self.freq += freq_diff;
        // }
        let modulator = self.lfo.run() * 5.0;

        self.freq = self.sequence[step_index];
        let freq = self.freq * TEMP_OCTAVE as f32; // + modulator;
                                                   // println!("Freq: {freq} modulator: {modulator}");
        self.vco.set_frequency(freq);
        value = self.vco.run();

        // Ramp between volumes
        if self.is_beat && self.ramp < 1.0 {
            self.ramp = clamp(self.ramp + 0.001, 0.0, 1.0);
        } else if !self.is_beat && self.ramp > 0.0 {
            self.ramp = clamp(self.ramp - 0.001, 0.0, 1.0);
        }

        value *= self.ramp;

        self.tick += 1;
        self.signal = value; // Assign the last value to signal

        value
    }

    // Current number of beats played, similar to elapsed time
    pub fn get_beat_index(&self) -> u32 {
        self.beat_index
    }

    // Used to make a sound or visualize
    pub fn show_beat(&self) -> bool {
        self.is_beat
    }

    pub fn get_signal(&self) -> f32 {
        self.signal
    }
}
