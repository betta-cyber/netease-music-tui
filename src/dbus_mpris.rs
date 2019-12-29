extern crate dbus;

use std::sync::Arc;
use dbus::blocking::Connection;
use dbus::tree::Factory;
use std::error::Error;
// use super::app::App;
// use super::handlers::TrackState;

pub fn dbus_mpris_server() -> Result<Connection, Box<dyn Error>> {
    // Let's start by starting up a connection to the session bus and request a name.
    let c = Connection::new_session()?;
    c.request_name("org.mpris.MediaPlayer2.ncmt", false, true, false)?;

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();

    // We create the signal first, since we'll need it in both inside the method callback
    // and when creating the tree.
    let signal = Arc::new(f.signal("HelloHappened", ()).sarg::<&str,_>("sender"));
    let signal2 = signal.clone();

    let method_next = {
        f.method("Next", (), move |m| {
            let name: &str = m.msg.read1()?;
            let s = format!("Hello {}!", name);
            let mret = m.msg.method_return().append1(s);
            // app_ins.skip_track(TrackState::Forword);
            Ok(vec!(mret))
            // Ok(vec![m.msg.method_return()])
        })
    };

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f.tree(()).add(f.object_path("/org/mpris/MediaPlayer2", ()).introspectable().add(

        // We add an interface to the object path...
        f.interface("org.mpris.MediaPlayer2.ncmt", ())
        .add_m(method_next)
        .add_s(signal2)

    // Also add the root path, to help introspection from debugging tools.
    )).add(f.object_path("/", ()).introspectable());

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive(&c);

    Ok(c)
    // Serve clients forever.
    // loop { c.process(Duration::from_millis(1000))?; }
}
