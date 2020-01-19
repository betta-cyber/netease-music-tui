use futures;
use futures::channel::oneshot;
// use futures::{future, Future};
use std::sync::mpsc::{RecvError, RecvTimeoutError, TryRecvError};
// use futures::channel::mpsc;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use super::sink::Sink;
use super::fetch::fetch_data;
use super::track::Track;
use std::sync::{Arc, Mutex};

use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum PlayerCommand {
    Load(Track, bool),
    Play,
    Pause,
    Stop,
    Seek(u32),
    Status,
    Volume(f32),
}


pub enum PlayerState {
    Stopped,
    Paused {
        start_of_track: oneshot::Sender<String>,
        end_of_track: oneshot::Sender<()>,
        normalisation_factor: f32,
        // stream_loader_controller: StreamLoaderController,
        bytes_per_second: usize,
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
    commands: Option<std::sync::mpsc::Sender<PlayerCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
    pub state: PlayerState,
    pub current: Option<Track>,
    pub sink: Arc<Mutex<rodio::Sink>>,
}

struct PlayerInternal {
    commands: std::sync::mpsc::Receiver<PlayerCommand>,
    // sink: Box<dyn Sink>,
    sink: Arc<Mutex<rodio::Sink>>,
    endpoint: rodio::Device,
    sink_running: bool,
    state: PlayerState,
    event_sender: futures::channel::mpsc::UnboundedSender<bool>,
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Started {
        track_url: String,
    },
    Changed {
        old_track_url: String,
        new_track_url: String,
    },
    Stopped {
        track_url: String,
    },
}

type PlayerEventChannel = futures::channel::mpsc::UnboundedReceiver<bool>;

// player
impl Player {
    // new player
    pub fn new<F>(
        // audio_filter: Option<Box<AudioFilter + Send>>,
        sink_builder: F,
    ) -> (Player, PlayerEventChannel)
    where
        F: FnOnce() -> Box<dyn Sink> + Send + 'static,
    {
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();
        let (event_sender, event_receiver) = futures::channel::mpsc::unbounded();

        let endpoint =
            rodio::default_output_device().expect("Failed to find default music endpoint");
        let sink = Arc::new(Mutex::new(rodio::Sink::new(&endpoint)));

        let internal = PlayerInternal {
            commands: cmd_rx,
            endpoint: endpoint,
            sink: sink.clone(),
            state: PlayerState::Stopped,
            sink_running: false,
            event_sender: event_sender,
        };

        let handle = thread::spawn(move || {
            internal.run();
        });

        (
            Player {
                commands: Some(cmd_tx),
                thread_handle: Some(handle),
                state: PlayerState::Stopped,
                current: None,
                sink: sink,
            },
            event_receiver,
        )
    }

    // run command
    fn command(&self, cmd: PlayerCommand) {
        self.commands.as_ref().expect("commands error").send(cmd).expect("send error");
    }

    pub fn load(
        &mut self,
        url: String,
        start_playing: bool,
    ) {

        let buffer = NamedTempFile::new().unwrap();
        let path = buffer.path().to_string_lossy().to_string();
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
                                        self.sink.lock().unwrap().stop();
                                        self.command(PlayerCommand::Load(track.clone(), start_playing));
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

    pub fn play(&self) {
        self.command(PlayerCommand::Play)
    }

    pub fn pause(&self) {
        self.command(PlayerCommand::Pause)
    }

    pub fn stop(&self) {
        self.command(PlayerCommand::Stop)
    }

    pub fn seek(&self, position_ms: u32) {
        self.command(PlayerCommand::Seek(position_ms));
    }

    pub fn status(&self) -> bool {
        self.state.is_playing()
    }

    pub fn set_volume(&self, volume: f32) {
        self.command(PlayerCommand::Volume(volume));
    }
}

// drop player
impl Drop for Player {
    fn drop(&mut self) {
        debug!("Shutting down player thread ...");
        self.commands = None;
        if let Some(handle) = self.thread_handle.take() {
            match handle.join() {
                Ok(_) => (),
                Err(_) => error!("Player thread panicked!"),
            }
        }
    }
}

// player internal
// loop for listen command
impl PlayerInternal {
    fn run(mut self) {
        loop {
            // debug!("loop");
            let cmd = if self.state.is_playing () {
                if self.sink_running {
                    match self.commands.try_recv() {
                        Ok(cmd) => Some(cmd),
                        Err(TryRecvError::Empty) => None,
                        Err(TryRecvError::Disconnected) => return,
                    }
                } else {
                    match self.commands.recv_timeout(Duration::from_secs(5)) {
                        Ok(cmd) => Some(cmd),
                        Err(RecvTimeoutError::Timeout) => None,
                        Err(RecvTimeoutError::Disconnected) => return,
                    }
                }
            } else {
                match self.commands.recv() {
                    Ok(cmd) => Some(cmd),
                    Err(RecvError) => return,
                }
            };
            if let Some(cmd) = cmd {
                self.handle_command(cmd);
            }
        }
    }

    fn handle_command(&mut self, cmd: PlayerCommand) {
        debug!("handle command={:#?}", cmd);
        match cmd {
            PlayerCommand::Load(track, start_playing) => {
                if start_playing {
                    let path = track.file.to_string_lossy().to_string();
                    self.start_sink(&path);
                }
            }
            PlayerCommand::Pause => {
                let sink = self.sink.lock().unwrap();
                sink.pause();
            }
            PlayerCommand::Stop => {
                let sink = self.sink.lock().unwrap();
                sink.stop();
            }
            PlayerCommand::Play => {
                let sink = self.sink.lock().unwrap();
                sink.play();
            }
            PlayerCommand::Volume(volume) => {
                let sink = self.sink.lock().unwrap();
                sink.set_volume(volume);
            }
            _ => {}
        }
        debug!("end this cmd");
    }

    fn start_sink(&mut self, path: &str) {
        self.sink = Arc::new(Mutex::new(rodio::Sink::new(&self.endpoint)));
        let sink = self.sink.lock().unwrap();

        let f = std::fs::File::open(&path).unwrap();
        let source = rodio::Decoder::new(std::io::BufReader::new(f)).unwrap();

        sink.append(source);
    }

    // fn send_event(&mut self, event: PlayerEvent) {
        // let _ = self.event_sender.unbounded_send(event.clone());
    // }
}

// drop PlayerInternal
impl Drop for PlayerInternal {
    fn drop(&mut self) {
        debug!("drop Player");
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
