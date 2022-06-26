use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType,AudioContextState,AudioNode};
use std::string::String;
/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct BinauralGenerator {
    audio_context: AudioContext,
    right_oscilator: web_sys::OscillatorNode,
    left_oscilator: web_sys::OscillatorNode,
    /// Overall gain (volume) control
    gain: web_sys::GainNode,
    fm_freq_ratio: f32,
    fm_gain_ratio: f32,
}

#[wasm_bindgen]
impl BinauralGenerator {
    pub fn new() -> Result<BinauralGenerator, JsValue> {
        let context = web_sys::AudioContext::new()?;

        // Create our web audio objects.
        let right = context.create_oscillator()?;
        let left = context.create_oscillator()?;
        let gain = context.create_gain()?;

        // Some initial settings:
        right.set_type(OscillatorType::Sine);
        right.frequency().set_value(107.0);
        left.set_type(OscillatorType::Sine);
        left.frequency().set_value(100.0);
        gain.gain().set_value(0.0); // starts muted

        //create splitter and merger audio nodes
        let merger = context.create_channel_merger_with_number_of_inputs(2)?;
        //connect oscilators with splitter
        right.connect_with_audio_node_and_output_and_input(&merger,0,1)?;
        left.connect_with_audio_node_and_output_and_input(&merger,0,0)?;
        merger.connect_with_audio_node(&gain);
        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&context.destination())?;
        // Start the oscillators!
        right.start()?;
        left.start()?;

        Ok( BinauralGenerator {
            audio_context: context,
            right_oscilator:right,
            left_oscilator: left,
            gain,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }
    /// Sets the gain for this oscillator, between 0.0 and 1.0.
    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_frequency(&self, freq: String, ear: &str) {
        if !freq.is_empty() {
            let f = freq.parse::<f32>().unwrap();
            if ear == "left" {
                self.left_oscilator.frequency().set_value(f);
            } else {
                self.right_oscilator.frequency().set_value(f);
            }
        }
    }

    #[wasm_bindgen]
    pub fn resume_suspend(&self) {
        if self.audio_context.state() == AudioContextState::Running {
            self.audio_context.suspend().ok();
        } else {
            self.audio_context.resume().ok();
        }
    }
/*
    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {

        self.set_primary_frequency(freq);
    }
 */
}

impl Drop for BinauralGenerator {
    fn drop(&mut self) {
        let _ = self.audio_context.close();
    }
}

