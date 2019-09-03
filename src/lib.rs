use gdnative::*;

#[cfg(unix)]
use speech_dispatcher::{Connection, Mode, Priority};
#[cfg(windows)]
use tolk::Tolk;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
struct TTS(
    #[cfg(unix)]
    Connection,
    #[cfg(windows)]
    Tolk,
);

#[methods]
impl TTS {
    fn _init(_owner: gdnative::Node) -> Self {
        #[cfg(unix)]
        {
            let connection = Connection::open("godot", "godot", "godot", Mode::Single);
            Self(connection)
        }
        #[cfg(windows)]
        Self(Tolk::new())
    }

    #[export]
    fn speak(&mut self, _owner: Node, message: GodotString, interrupt: bool) {
        let message = message.to_string();
        println!("{}: {}", message, interrupt);
        #[cfg(unix)]
        {
            if interrupt {
                self.0.cancel();
            }
            if message != "" {
                self.0.say(Priority::Important, message);
            }
        }
    }

    #[export]
    fn stop(&mut self, _owner: Node) {
        #[cfg(unix)]
        {
            self.0.cancel();
        }
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<TTS>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
