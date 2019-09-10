use gdnative::*;

use tts::{TTS as Tts};

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
struct TTS(Tts);

#[methods]
impl TTS {
    fn _init(_owner: gdnative::Node) -> Self {
        let tts = Tts::default().unwrap();
        Self(tts)
    }

    #[export]
    fn speak(&mut self, _owner: Node, message: GodotString, interrupt: bool) {
        let message = message.to_string();
        println!("{}: {}", message, interrupt);
        self.0.speak(message, interrupt).unwrap();
    }

    #[export]
    fn stop(&mut self, _owner: Node) {
        self.0.stop().unwrap();
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<TTS>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
