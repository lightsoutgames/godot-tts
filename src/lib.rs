use std::u8;

use gdnative::*;
use gdnative::init::*;
use tts::{Features, TTS as Tts};

struct TTS(Tts);

impl NativeClass for TTS {
    type Base = Node;
    type UserData = user_data::MutexData<TTS>;

    fn class_name() -> &'static str {
        "TTS"
    }

    fn init(owner: Self::Base) -> Self {
        Self::_init(owner)
    }

    fn register_properties(builder: &ClassBuilder<Self>) {
        builder.add_property(Property {
            name: "rate",
            default: 50,
            hint: PropertyHint::Range {
                range: 0.0..100.0,
                step: 1.,
                slider: true,
            },
            getter: |this: &TTS| {
                let rate = this.0.get_rate().unwrap();
                rate / u8::MAX * 100
            },
            setter: |this: &mut TTS, mut v: u8| {
                if v > 100 {
                    v = 100;
                }
                let mut v = v as f32;
                v = v * u8::MAX as f32 / 100.;
                this.0.set_rate(v as u8).unwrap();
            },
            usage: PropertyUsage::DEFAULT,
        });
    }
}

#[methods]
impl TTS {
    fn _init(_owner: gdnative::Node) -> Self {
        let tts = Tts::default().unwrap();
        Self(tts)
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
            rate: rate_feature, ..
        } = self.0.supported_features();
        rate_feature
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<TTS>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
