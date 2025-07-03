// frontend_rust/src/audio/mod.rs

// This module will handle loading audio files, feeding data to projectM, etc.

const SAMPLE_RATE: u32 = 44100; // Standard sample rate
const CHANNELS: u16 = 2;       // Stereo
const BUFFER_SIZE_SAMPLES: usize = 1024; // Number of samples per channel per buffer

// Simple sine wave generator state
struct SineWaveGenerator {
    phase: f32,
    frequency: f32,
}

impl SineWaveGenerator {
    fn new(frequency: f32) -> Self {
        SineWaveGenerator { phase: 0.0, frequency }
    }

    fn next_sample(&mut self) -> f32 {
        let value = (self.phase * std::f32::consts::TAU).sin();
        self.phase += self.frequency / SAMPLE_RATE as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        value * 0.5 // Amplitude
    }
}

// Global or passed-around generator state
// For simplicity in this placeholder, let's make it static.
// In a real app, this would be part of a struct or managed differently.
static mut SINE_GENERATOR_L: Option<SineWaveGenerator> = None;
static mut SINE_GENERATOR_R: Option<SineWaveGenerator> = None;


pub fn init_audio_placeholder() {
    unsafe {
        SINE_GENERATOR_L = Some(SineWaveGenerator::new(440.0)); // A4 note for left
        SINE_GENERATOR_R = Some(SineWaveGenerator::new(660.0)); // E5 note for right
    }
    println!("Placeholder audio initialized (sine wave generators).");
}

/// Generates a buffer of stereo PCM f32 audio data.
/// Returns a Vec<f32> with interleaved samples [L, R, L, R, ...].
pub fn get_placeholder_audio_buffer() -> Vec<f32> {
    let mut buffer = Vec::with_capacity(BUFFER_SIZE_SAMPLES * CHANNELS as usize);
    unsafe {
        if SINE_GENERATOR_L.is_none() || SINE_GENERATOR_R.is_none() {
            init_audio_placeholder(); // Ensure initialized
        }

        if let (Some(gen_l), Some(gen_r)) = (SINE_GENERATOR_L.as_mut(), SINE_GENERATOR_R.as_mut()) {
            for _ in 0..BUFFER_SIZE_SAMPLES {
                buffer.push(gen_l.next_sample());
                buffer.push(gen_r.next_sample());
            }
        }
    }
    buffer
}

// Kept for compatibility with main.rs call, though not used by placeholder directly yet.
pub fn init_audio() {
    init_audio_placeholder();
}
