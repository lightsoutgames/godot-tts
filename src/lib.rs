#[macro_use]
extern crate gdnative as godot;

#[cfg(unix)]
extern crate speech_dispatcher;
#[cfg(windows)]
extern crate tolk;

#[cfg(unix)]
use speech_dispatcher::{Connection, Mode, Priority};
#[cfg(windows)]
use tolk::Tolk;

godot_class! {
    class TTS: godot::Object {
        fields {
            #[cfg(unix)]
            connection: Connection,
            #[cfg(windows)]
            tolk: Tolk,
        }

        setup(_builder) {
        }

        constructor(header) {
            TTS {
                header,
                #[cfg(unix)]
                connection: Connection::open("godot", "godot", "godot", Mode::Single),
                #[cfg(windows)]
                tolk: Tolk::new(),
            }
        }

        export fn speak(&mut self) {
            #[cfg(unix)]
            {
                self.connection.say(Priority::Important, "Hello, world.".to_string());
            }
        }
    }
}

fn init(handle: godot::init::InitHandle) {
    TTS::register_class(handle);
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
