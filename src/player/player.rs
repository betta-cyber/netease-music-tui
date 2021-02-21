use futures::channel::oneshot;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use super::fetch::fetch_data;
use super::track::Track;

use std::thread;
use std::fs;

#[allow(unused)]
pub enum PlayerState {
    Stopped,
    Paused {
        // start_of_track: oneshot::Sender<String>,
        // end_of_track: oneshot::Sender<()>,
        // normalisation_factor: f32,
        // stream_loader_controller: StreamLoaderController,
        // bytes_per_second: usize,
    },
    Playing {
        // start_of_track: oneshot::Sender<String>,
        // end_of_track: oneshot::Sender<()>,
        // normalisation_factor: f32,
        // // stream_loader_controller: StreamLoaderController,
        // bytes_per_second: usize,
    },
    EndOfTrack {
        url: String,
    },
    Invalid,
}

pub struct Player {
    // commands: Option<std::sync::mpsc::Sender<PlayerCommand>>,
    // endpoint: rodio::Device,
    pub state: PlayerState,
    pub current: Option<Track>,
    pub sink: rodio::Sink,
}

// player
impl Player {
    // new player
    pub fn new<>(
        // audio_filter: Option<Box<AudioFilter + Send>>,
        // sink_builder: F,
    ) -> Player
    // where
        // F: FnOnce() -> Box<dyn Sink> + Send + 'static,
    {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        // let endpoint =
            // rodio::default_output_device().expect("Failed to find default music endpoint");
        // let sink = rodio::Sink::new(&endpoint);

        Player {
            state: PlayerState::Stopped,
            current: None,
            sink: sink,
            // endpoint: endpoint,
        }
    }

    // run command
    // fn command(&self, cmd: PlayerCommand) {
        // self.commands.as_ref().expect("commands error").send(cmd).expect("send error");
    // }

    pub fn load(
        &mut self,
        url: String,
        start_playing: bool,
    ) {
        match &self.current {
            Some(track) => {
                fs::remove_file(track.file()).ok();
                self.start();
            }
            None => {}
        }

        let buffer = NamedTempFile::new().unwrap();
        let path = buffer.path().to_string_lossy().to_string();
        debug!("{:#?}", path);
        let pathbuf = PathBuf::from(path);

        let (ptx, mut prx) = oneshot::channel::<String>();

        thread::spawn(move || {
            fetch_data(&url.to_owned(), buffer, ptx).expect("error thread task");
        });
        if start_playing {
            loop {
                match prx.try_recv() {
                    Ok(p) => {
                        match p {
                            Some(_) => {
                                match Track::load(pathbuf) {
                                    Ok(track) => {
                                        let mut track = track;
                                        self.load_track(track.clone(), start_playing);
                                        track.resume();
                                        self.current = Some(track);
                                        self.state = PlayerState::Playing{};
                                    }
                                    Err(_) => {}
                                }
                                break;
                            }
                            None => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }

    pub fn load_track(&mut self, track: Track, playing: bool) {
        if playing {
            let path = track.file.to_string_lossy().to_string();
            let f = std::fs::File::open(&path).unwrap();
            let source = rodio::Decoder::new(std::io::BufReader::new(f)).unwrap();

            self.sink.play();
            self.sink.append(source);
        }
    }

    pub fn start(&mut self) {
        let vol = self.sink.volume();
        self.sink.stop();
        // self.sink = rodio::Sink::new(&self.endpoint);
        self.set_volume(vol);
    }

    pub fn play(&mut self) {
        self.sink.play();
        self.state = PlayerState::Playing{};
        self.current = self.current.take().and_then(|mut s| {
            s.resume();
            Some(s)
        });
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.state = PlayerState::Paused{};
        self.current = self.current.take().and_then(|mut s| {
            s.stop();
            Some(s)
        });
    }

    pub fn stop(&self) {
        self.sink.stop()
    }

    #[allow(unused)]
    pub fn seek(&self, position_ms: u32) {
        // self.command(PlayerCommand::Seek(position_ms));
    }

    pub fn status(&self) -> bool {
        self.state.is_playing()
    }

    pub fn get_volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume)
    }
}

// drop player
impl Drop for Player {
    fn drop(&mut self) {
        debug!("Shutting down player thread ...");
        // remove cache file
    }
}

// player state
impl PlayerState {
    fn is_playing(&self) -> bool {
        use self::PlayerState::*;
        match *self {
            Stopped | EndOfTrack { .. } | Paused { .. } => false,
            Playing { .. } => true,
            Invalid => panic!("invalid state"),
        }
    }
}
