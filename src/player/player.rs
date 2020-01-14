use futures;
use futures::channel::oneshot;
use futures::{future, Future};
use std::sync::mpsc::{RecvError, RecvTimeoutError, TryRecvError};
use futures::channel::mpsc;
use futures::executor::block_on;
use std::sync::Arc;
use tempfile::NamedTempFile;
use super::sink::Sink;
use super::fetch::fetch_data;

use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum PlayerCommand {
    Load(String, bool, oneshot::Sender<String>),
    Play,
    Pause,
    Stop,
    Seek(u32),
}

enum PlayerState {
    Stopped,
    Paused {
        start_of_track: oneshot::Sender<String>,
        end_of_track: oneshot::Sender<()>,
        normalisation_factor: f32,
        // stream_loader_controller: StreamLoaderController,
        bytes_per_second: usize,
    },
    Playing {
        start_of_track: oneshot::Sender<String>,
        end_of_track: oneshot::Sender<()>,
        normalisation_factor: f32,
        // stream_loader_controller: StreamLoaderController,
        bytes_per_second: usize,
    },
    EndOfTrack {
        url: String,
    },
    Invalid,
}

pub struct Player {
    commands: Option<std::sync::mpsc::Sender<PlayerCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

struct PlayerInternal {
    commands: std::sync::mpsc::Receiver<PlayerCommand>,

    state: PlayerState,
    sink: Box<dyn Sink>,
    sink_running: bool,
    event_sender: futures::channel::mpsc::UnboundedSender<PlayerEvent>,
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

type PlayerEventChannel = futures::channel::mpsc::UnboundedReceiver<PlayerEvent>;

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

        let handle = thread::spawn(move || {
            let internal = PlayerInternal {
                commands: cmd_rx,

                state: PlayerState::Stopped,
                sink: sink_builder(),
                sink_running: false,
                // audio_filter: audio_filter,
                event_sender: event_sender,
            };

            internal.run();
        });
        // handle.join().expect("error create thread");
        // debug!("internal init");

        (
            Player {
                commands: Some(cmd_tx),
                thread_handle: Some(handle),
            },
            event_receiver,
        )
    }

    // run command
    fn command(&self, cmd: PlayerCommand) {
        self.commands.as_ref().expect("commands error").send(cmd).expect("send error");
        debug!("cmd do finish");
    }

    pub fn load(
        &self,
        url: &str,
        start_playing: bool,
    ) {
        let (tx, rx) = oneshot::channel::<String>();
        self.command(PlayerCommand::Load(url.to_owned(), start_playing, tx));
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
            debug!("loop");
            let cmd = if self.state.is_playing() {
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
            if self.sink_running {
                return;
            }
            // debug!("cmd {:#?}", cmd);
            if let Some(cmd) = cmd {
                self.handle_command(cmd);
            }
            debug!("this loop end");
        }
    }

    fn handle_command(&mut self, cmd: PlayerCommand) {
        debug!("handle command={:#?}", cmd);
        match cmd {
            PlayerCommand::Load(url, start_playing, start_tx) => {
                if self.state.is_playing() {
                    // self.stop_sink_if_running();
                    debug!("is playing");
                }
                // new thread for download file
                let mut buffer = NamedTempFile::new().unwrap();
                let path = buffer.path().to_string_lossy().to_string();

                debug!("buffer {}", path);

                thread::spawn(move || {
                    fetch_data(&url, buffer, start_tx).expect("error thread task");
                });
                // load and autoplaying
                if start_playing {
                    thread::sleep(Duration::from_millis(1000));
                    // let path = start_rx.try_recv().unwrap().unwrap();
                    debug!("append to sink");
                    self.sink.append(&path);
                }
                // self.sink.append();
            }
            PlayerCommand::Pause => {
                self.sink.pause();
            }
            PlayerCommand::Stop => {
                self.sink.stop();
            }
            PlayerCommand::Play => {
                self.sink.start();
            }
            _ => {}
        }
        debug!("end this cmd");
    }

    fn start_sink(&mut self) {
        match self.sink.start() {
            Ok(()) => self.sink_running = true,
            Err(err) => error!("Could not start audio: {}", err),
        }
    }

    fn send_event(&mut self, event: PlayerEvent) {
        let _ = self.event_sender.unbounded_send(event.clone());
    }
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
