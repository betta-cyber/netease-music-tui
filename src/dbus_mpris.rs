// dbus-send example
// for get Properties
// dbus-send --session --print-reply --dest=org.mpris.MediaPlayer2.ncmt /org/mpris/MediaPlayer2 org.freedesktop.DBus.Properties.Get string:"org.mpris.MediaPlayer2.ncmt" string:"Rate"
// for method
// dbus-send --session --print-reply --dest=org.mpris.MediaPlayer2.ncmt /org/mpris/MediaPlayer2 org.mpris.MediaPlayer2.ncmt.Next  string:"betta"
extern crate dbus;

use dbus::blocking::Connection;
use dbus::tree::{Factory, Access};
use std::error::Error;
use std::sync::mpsc::Sender;
use std::time::Duration;
use super::app::App;
use super::handlers::TrackState;
use super::player::PlayerCommand;

pub fn dbus_mpris_server(tx: Sender<PlayerCommand>) -> Result<(), Box<dyn Error>> {
    // Let's start by starting up a connection to the session bus and request a name.
    let mut c = Connection::new_session()?;
    c.request_name("org.mpris.MediaPlayer2.ncmt", false, true, false)?;

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();

    let method_next = {
        let local_tx = tx.clone();
        f.method("Next", (), move |m| {
            local_tx.send(PlayerCommand::Next).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_previous = {
        let local_tx = tx.clone();
        f.method("Previous", (), move |m| {
            local_tx.send(PlayerCommand::Previous).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_pause = {
        let local_tx = tx.clone();
        f.method("Pause", (), move |m| {
            local_tx.send(PlayerCommand::Pause).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_play_pause = {
        let local_tx = tx.clone();
        f.method("PlayPause", (), move |m| {
            local_tx.send(PlayerCommand::PlayPause).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_stop = {
        let local_tx = tx.clone();
        f.method("Stop", (), move |m| {
            local_tx.send(PlayerCommand::Stop).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_play = {
        let local_tx = tx.clone();
        f.method("Play", (), move |m| {
            local_tx.send(PlayerCommand::Play).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_seek = {
        let local_tx = tx.clone();
        f.method("Seek", (), move |m| {
            local_tx.send(PlayerCommand::Seek(5)).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_set_position = {
        let local_tx = tx.clone();
        f.method("SetPosition", (), move |m| {
            let (track_id, position) = m.msg.read2()?;
            local_tx.send(PlayerCommand::Position(track_id, position)).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let method_open_uri = {
        let local_tx = tx.clone();
        f.method("OpenUri", (), move |m| {
            let uri = m.msg.read1()?;
            local_tx.send(PlayerCommand::Load(uri)).unwrap();
            Ok(vec![m.msg.method_return()])
        })
    };

    let property_playback_status = f
        .property::<String, _>("PlaybackStatus", ())
        .access(Access::Read)
        .on_get(|iter, _| {
            iter.append(1.0);
            Ok(())
        });


    let property_rate = f
        .property::<f64, _>("Rate", ())
        .access(Access::Read)
        .on_get(|iter, _| {
            iter.append(1.0);
            Ok(())
        });

    let property_max_rate = f
        .property::<f64, _>("MaximumRate", ())
        .access(Access::Read)
        .on_get(|iter, _| {
            iter.append(1.0);
            Ok(())
        });

    let property_min_rate = f
        .property::<f64, _>("MinimumRate", ())
        .access(Access::Read)
        .on_get(|iter, _| {
            iter.append(1.0);
            Ok(())
        });

    // let property_loop_status = f
        // .property::<String, _>("LoopStatus", ())
        // .access(Access::Read)
        // .on_get(
        // ));

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f
        .tree(())
        .add(
            f.object_path("/org/mpris/MediaPlayer2", ())
                .introspectable()
                .add(
                    f.interface("org.mpris.MediaPlayer2.ncmt", ())
                        .add_m(method_next)
                        .add_m(method_previous)
                        .add_m(method_pause)
                        .add_m(method_play_pause)
                        .add_m(method_stop)
                        .add_m(method_play)
                        .add_m(method_seek)
                        .add_m(method_set_position)
                        .add_m(method_open_uri)
                        .add_p(property_playback_status)
                        .add_p(property_rate)
                        .add_p(property_max_rate)
                        .add_p(property_min_rate)
                ),
        )
        .add(f.object_path("/", ()).introspectable());

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive(&c);

    // Serve clients forever.
    loop {
        c.process(Duration::from_nanos(1))?;
    }
}

pub fn dbus_mpris_handler(r: PlayerCommand, app: &mut App) {
    match r {
        PlayerCommand::Next => {
            app.skip_track(TrackState::Forword);
        }
        PlayerCommand::Previous => {
            app.skip_track(TrackState::Backword);
        }
        PlayerCommand::Pause => {
            app.player.pause();
        }
        PlayerCommand::PlayPause => {
            app.player.play();
        }
        PlayerCommand::Stop => {
            app.player.stop();
        }
        PlayerCommand::Play => {
            app.player.play();
        }
        PlayerCommand::Seek(x) => {
            app.player.seek(x);
        }
        PlayerCommand::Position(_track_id, position) => {
            app.player.position(position);
        }
        PlayerCommand::Load(uri) => {
            app.player.play_url(&uri);
        }
        _ => {}
    }
}
