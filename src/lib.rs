use std::u8;

use gdnative::init::*;
use gdnative::*;
use tts::{Features, TTS as Tts};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_properties)]
struct TTS(Tts);

#[methods]
impl TTS {
    fn _init(_owner: gdnative::Node) -> Self {
        let tts = Tts::default().unwrap();
        Self(tts)
    }

    fn register_properties(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<u8>("rate")
            .with_default(50)
            .with_getter(|this: &TTS, _| match this.0.get_rate() {
                Ok(rate) => rate / u8::MAX * 100,
                _ => 0,
            })
            .with_setter(|this: &mut TTS, _, mut v: u8| {
                if v > 100 {
                    v = 100;
                }
                let mut v = v as f32;
                v = v * u8::MAX as f32 / 100.;
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    this.0.set_rate(v as u8).unwrap();
                }
            })
            .done()
    }

    #[export]
    fn speak(&mut self, _owner: Node, message: GodotString, interrupt: bool) {
        let message = message.to_string();
        self.0.speak(message, interrupt).unwrap();
    }

    #[export]
    fn stop(&mut self, _owner: Node) {
        self.0.stop().unwrap();
    }

    #[export]
    fn is_rate_supported(&mut self, _owner: Node) -> bool {
        let Features {
            rate: rate_supported,
            ..
        } = self.0.supported_features();
        rate_supported
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<TTS>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
