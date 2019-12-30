extern crate dbus;

use dbus::blocking::Connection;
use dbus::tree::Factory;
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

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f
        .tree(())
        .add(
            f.object_path("/org/mpris/MediaPlayer2", ())
                .introspectable()
                .add(
                    f.interface("org.mpris.MediaPlayer2.ncmt", ())
                        .add_m(method_next)
                        .add_m(method_previous),
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
        _ => {}
    }
}
