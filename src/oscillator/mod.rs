//! 
//! Synthesis Oscillator module.
//!

pub use self::waveform::Waveform;
pub use self::amplitude::Amplitude;
pub use self::amplitude::Envelope as AmpEnvelope;
pub use self::frequency::Frequency;
pub use self::frequency::Envelope as FreqEnvelope;
pub use self::freq_warp::FreqWarp;

pub mod waveform;
pub mod amplitude;
pub mod frequency;
pub mod freq_warp;


/// The fundamental component of a synthesizer.
#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Oscillator<W, A, F, FW> {
    /// Waveform used for phase movement.
    pub waveform: W,
    /// Envelope for amplitude interpolation.
    pub amplitude: A,
    /// Envelope for frequency interpolation.
    pub frequency: F,
    /// A type used for warping the Oscillator's frequency.
    pub freq_warp: FW,
    /// Whether or not the Oscillator is currently muted.
    pub is_muted: bool,
}


impl<W, A, F, FW> Oscillator<W, A, F, FW> {

    /// Oscillator constructor.
    #[inline]
    pub fn new(waveform: W, amplitude: A, frequency: F, freq_warp: FW) -> Oscillator<W, A, F, FW> {
        Oscillator {
            waveform: waveform,
            amplitude: amplitude,
            frequency: frequency,
            freq_warp: freq_warp,
            is_muted: false,
        }
    }

    /// Waveform builder method.
    #[inline]
    pub fn waveform<WNew>(self, waveform: WNew) -> Oscillator<WNew, A, F, FW> {
        let Oscillator { amplitude, frequency, freq_warp, is_muted, .. } = self;
        Oscillator {
            waveform: waveform,
            amplitude: amplitude, 
            frequency: frequency,
            freq_warp: freq_warp,
            is_muted: is_muted,
        }
    }

    /// Amplitude envelope builder method.
    #[inline]
    pub fn amplitude<ANew>(self, amplitude: ANew) -> Oscillator<W, ANew, F, FW> {
        let Oscillator { waveform, frequency, freq_warp, is_muted, .. } = self;
        Oscillator {
            waveform: waveform,
            amplitude: amplitude, 
            frequency: frequency,
            freq_warp: freq_warp,
            is_muted: is_muted,
        }
    }

    /// Amplitude envelope builder method.
    #[inline]
    pub fn frequency<FNew>(self, frequency: FNew) -> Oscillator<W, A, FNew, FW> {
        let Oscillator { waveform, amplitude, freq_warp, is_muted, .. } = self;
        Oscillator {
            waveform: waveform,
            amplitude: amplitude, 
            frequency: frequency,
            freq_warp: freq_warp,
            is_muted: is_muted,
        }
    }

    /// Calculate and return the amplitude at the given ratio.
    #[inline]
    pub fn amp_at(&self, phase: f64, playhead_perc: f64) -> f32 where
        A: Amplitude,
        W: Waveform,
    {
        self.waveform.amp_at_phase(phase) * self.amplitude.amp_at_playhead(playhead_perc)
    }

    /// Calculate and return the phase that should follow some given phase.
    #[inline]
    pub fn next_phase(&self,
                      phase: f64,
                      playhead_perc: f64,
                      note_freq_multi: f64,
                      sample_hz: f64,
                      freq_warp_phase: &mut f64) -> f64 where
        W: Waveform,
        F: Frequency,
        FW: FreqWarp,
    {
        let hz = self.frequency.hz_at_playhead(playhead_perc);
        let hz = self.waveform.process_hz(hz);
        self.freq_warp.step_phase(sample_hz, freq_warp_phase);
        let warped_hz = self.freq_warp.warp_hz(hz, *freq_warp_phase);
        let note_hz = warped_hz * note_freq_multi;
        phase + (note_hz / sample_hz)
    }

}

