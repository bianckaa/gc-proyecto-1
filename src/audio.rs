use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;

use rodio::buffer::SamplesBuffer;
use rodio::decoder::LoopedDecoder;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

const SAMPLE_RATE: u32 = 44100;

pub const MENU_MUSIC: &str = "assets/audio/menu.mp3";
const VICTORY_PATH: &str = "assets/audio/victoria.wav";

struct LoopedTrack {
    samples: Arc<Vec<f32>>,
    index: usize,
}

impl Iterator for LoopedTrack {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.samples.is_empty() {
            return None;
        }
        let sample = self.samples[self.index];
        self.index = (self.index + 1) % self.samples.len();
        Some(sample)
    }
}

impl Source for LoopedTrack {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn envelope(position: f32, length: f32) -> f32 {
    let attack = 0.02;
    let release = 0.25;
    if position < attack {
        position / attack
    } else if position > length - release {
        ((length - position) / release).max(0.0)
    } else {
        1.0
    }
}

fn square(phase: f32) -> f32 {
    if phase.sin() >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

fn add_note(buffer: &mut [f32], start: f32, length: f32, freq: f32, gain: f32, bright: bool) {
    if freq <= 0.0 {
        return;
    }
    let first = (start * SAMPLE_RATE as f32) as usize;
    let count = (length * SAMPLE_RATE as f32) as usize;
    for n in 0..count {
        let index = first + n;
        if index >= buffer.len() {
            break;
        }
        let t = n as f32 / SAMPLE_RATE as f32;
        let phase = 2.0 * std::f32::consts::PI * freq * t;
        let wave = if bright {
            0.6 * phase.sin() + 0.4 * square(phase)
        } else {
            phase.sin()
        };
        buffer[index] += wave * gain * envelope(t, length);
    }
}

fn normalize(buffer: &mut [f32], target: f32) {
    let peak = buffer.iter().fold(0.0f32, |acc, s| acc.max(s.abs()));
    if peak > 0.0001 {
        let factor = target / peak;
        for sample in buffer.iter_mut() {
            *sample *= factor;
        }
    }
}

fn build_music() -> Vec<f32> {
    let beat = 0.45;
    let bars = 8;
    let total = beat * 4.0 * bars as f32;
    let mut buffer = vec![0.0f32; (total * SAMPLE_RATE as f32) as usize + 1];

    let chords = [
        [130.81, 164.81, 196.00],
        [196.00, 246.94, 293.66],
        [220.00, 261.63, 329.63],
        [174.61, 220.00, 261.63],
    ];
    let melody = [
        523.25, 493.88, 440.00, 493.88, 587.33, 523.25, 440.00, 392.00, 440.00, 493.88, 523.25,
        587.33, 659.25, 587.33, 523.25, 493.88, 523.25, 587.33, 659.25, 587.33, 523.25, 440.00,
        392.00, 440.00, 493.88, 523.25, 587.33, 523.25, 493.88, 440.00, 392.00, 349.23,
    ];

    for bar in 0..bars {
        let chord = chords[bar % chords.len()];
        let bar_start = bar as f32 * beat * 4.0;
        for (voice, freq) in chord.iter().enumerate() {
            add_note(
                &mut buffer,
                bar_start,
                beat * 3.8,
                *freq,
                0.16 - voice as f32 * 0.02,
                false,
            );
        }
        for step in 0..4 {
            add_note(
                &mut buffer,
                bar_start + step as f32 * beat,
                beat * 0.5,
                chord[0] / 2.0,
                0.22,
                false,
            );
        }
        for step in 0..4 {
            let note = melody[(bar * 4 + step) % melody.len()];
            add_note(
                &mut buffer,
                bar_start + step as f32 * beat,
                beat * 0.85,
                note,
                0.13,
                true,
            );
        }
    }

    normalize(&mut buffer, 0.7);
    buffer
}

fn build_thud() -> Vec<f32> {
    let length = 0.20;
    let count = (length * SAMPLE_RATE as f32) as usize;
    let mut buffer = vec![0.0f32; count];
    let mut state: u32 = 12345;
    for (n, sample) in buffer.iter_mut().enumerate() {
        let t = n as f32 / SAMPLE_RATE as f32;
        let decay = (-18.0 * t).exp();
        let phase = 2.0 * std::f32::consts::PI * 78.0 * t;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = ((state >> 16) as f32 / 32768.0) - 1.0;
        *sample = (phase.sin() * 0.8 + noise * 0.25) * decay;
    }
    normalize(&mut buffer, 0.85);
    buffer
}

fn build_victory() -> Vec<f32> {
    let length = 1.1;
    let mut buffer = vec![0.0f32; (length * SAMPLE_RATE as f32) as usize + 1];
    let notes = [523.25, 659.25, 783.99, 1046.50];
    for (step, freq) in notes.iter().enumerate() {
        add_note(&mut buffer, step as f32 * 0.13, 0.55, *freq, 0.4, true);
    }
    add_note(&mut buffer, 0.52, 0.55, 1318.51, 0.35, true);
    normalize(&mut buffer, 0.8);
    buffer
}

fn open_looped(path: &str) -> Option<LoopedDecoder<BufReader<File>>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("no se encontro el archivo de audio {path}, se usa la pista generada por codigo");
            return None;
        }
    };
    match Decoder::new_looped(BufReader::new(file)) {
        Ok(decoder) => Some(decoder),
        Err(_) => {
            println!("no se pudo decodificar {path}, se usa la pista generada por codigo");
            None
        }
    }
}

fn open_once(path: &str) -> Option<Decoder<BufReader<File>>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("no se encontro el archivo de audio {path}, se usa el efecto generado por codigo");
            return None;
        }
    };
    match Decoder::new(BufReader::new(file)) {
        Ok(decoder) => Some(decoder),
        Err(_) => {
            println!("no se pudo decodificar {path}, se usa el efecto generado por codigo");
            None
        }
    }
}

pub struct Audio {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    music: Option<Sink>,
    victory_sink: Option<Sink>,
    current_track: String,
    music_samples: Arc<Vec<f32>>,
    thud: Arc<Vec<f32>>,
    victory: Arc<Vec<f32>>,
}

impl Audio {
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => Audio {
                _stream: Some(stream),
                handle: Some(handle),
                music: None,
                victory_sink: None,
                current_track: String::new(),
                music_samples: Arc::new(build_music()),
                thud: Arc::new(build_thud()),
                victory: Arc::new(build_victory()),
            },
            Err(_) => {
                println!("no se pudo iniciar el audio, el juego continua en silencio");
                Audio {
                    _stream: None,
                    handle: None,
                    music: None,
                    victory_sink: None,
                    current_track: String::new(),
                    music_samples: Arc::new(Vec::new()),
                    thud: Arc::new(Vec::new()),
                    victory: Arc::new(Vec::new()),
                }
            }
        }
    }

    pub fn play_music(&mut self, path: &str) {
        self.stop_victory();
        if self.current_track == path {
            return;
        }
        self.stop_music();
        let handle = match &self.handle {
            Some(handle) => handle,
            None => return,
        };
        let sink = match Sink::try_new(handle) {
            Ok(sink) => sink,
            Err(_) => return,
        };
        sink.set_volume(0.35);
        match open_looped(path) {
            Some(decoder) => sink.append(decoder),
            None => sink.append(LoopedTrack {
                samples: Arc::clone(&self.music_samples),
                index: 0,
            }),
        }
        self.music = Some(sink);
        self.current_track = path.to_string();
    }

    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music.take() {
            sink.stop();
        }
        self.current_track.clear();
    }

    fn play_once(&self, samples: &Arc<Vec<f32>>, volume: f32) {
        let handle = match &self.handle {
            Some(handle) => handle,
            None => return,
        };
        if samples.is_empty() {
            return;
        }
        let buffer = SamplesBuffer::new(1, SAMPLE_RATE, samples.as_slice().to_vec());
        let _ = handle.play_raw(buffer.amplify(volume));
    }

    pub fn play_thud(&self) {
        self.play_once(&self.thud, 0.7);
    }

    pub fn stop_victory(&mut self) {
        if let Some(sink) = self.victory_sink.take() {
            sink.stop();
        }
    }

    pub fn play_victory(&mut self) {
        let handle = match &self.handle {
            Some(handle) => handle,
            None => return,
        };
        let sink = match Sink::try_new(handle) {
            Ok(sink) => sink,
            Err(_) => return,
        };
        sink.set_volume(0.9);
        match open_once(VICTORY_PATH) {
            Some(decoder) => sink.append(decoder),
            None => {
                if self.victory.is_empty() {
                    return;
                }
                sink.append(SamplesBuffer::new(
                    1,
                    SAMPLE_RATE,
                    self.victory.as_slice().to_vec(),
                ));
            }
        }
        self.victory_sink = Some(sink);
    }
}
