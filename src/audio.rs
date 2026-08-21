use std::fs::File;
use std::io::BufReader;

use rodio::decoder::LoopedDecoder;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub const MENU_MUSIC: &str = "assets/audio/menu.mp3";
const VICTORY_PATH: &str = "assets/audio/victoria.wav";

fn open_looped(path: &str) -> Option<LoopedDecoder<BufReader<File>>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("no se encontro el archivo de audio {path}");
            return None;
        }
    };
    match Decoder::new_looped(BufReader::new(file)) {
        Ok(decoder) => Some(decoder),
        Err(_) => {
            println!("no se pudo decodificar {path}");
            None
        }
    }
}

fn open_once(path: &str) -> Option<Decoder<BufReader<File>>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("no se encontro el archivo de audio {path}");
            return None;
        }
    };
    match Decoder::new(BufReader::new(file)) {
        Ok(decoder) => Some(decoder),
        Err(_) => {
            println!("no se pudo decodificar {path}");
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
            },
            Err(_) => {
                println!("no se pudo iniciar el audio, el juego continua en silencio");
                Audio {
                    _stream: None,
                    handle: None,
                    music: None,
                    victory_sink: None,
                    current_track: String::new(),
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
        if let Some(decoder) = open_looped(path) {
            sink.append(decoder);
            self.music = Some(sink);
            self.current_track = path.to_string();
        }
    }

    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music.take() {
            sink.stop();
        }
        self.current_track.clear();
    }

    pub fn play_thud(&self) {
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
        if let Some(decoder) = open_once(VICTORY_PATH) {
            sink.append(decoder);
            self.victory_sink = Some(sink);
        }
    }
}
