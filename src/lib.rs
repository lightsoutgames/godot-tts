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
            .add_property("rate")
            .with_getter(|this: &TTS, _| match this.0.get_rate() {
                Ok(rate) => rate,
                _ => 0.,
            })
            .with_setter(|this: &mut TTS, _, v: f32| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    let mut v = v;
                    if v < this.0.min_rate() {
                        v = this.0.min_rate();
                    } else if v > this.0.max_rate() {
                        v = this.0.max_rate();
                    }
                    this.0.set_rate(v).unwrap();
                }
            })
            .done();
        builder
            .add_property("min_rate")
            .with_getter(|this: &TTS, _| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    this.0.min_rate()
                } else {
                    0.
                }
            })
            .done();
        builder
            .add_property("max_rate")
            .with_getter(|this: &TTS, _| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    this.0.max_rate()
                } else {
                    0.
                }
            })
            .done();
        builder
            .add_property("normal_rate")
            .with_getter(|this: &TTS, _| {
                let Features {
                    rate: rate_supported,
                    ..
                } = this.0.supported_features();
                if rate_supported {
                    this.0.normal_rate()
                } else {
                    0.
                }
            })
            .done();
        builder
            .add_property("can_detect_screen_reader")
            .with_getter(|_: &TTS, _| {
                #[cfg(windows)]
                return true;
                return false;
            })
            .done();
        builder
            .add_property("has_screen_reader")
            .with_getter(|_: &TTS, _| {
                #[cfg(windows)]
                {
                    let tolk = tolk::Tolk::new();
                    return tolk.detect_screen_reader().is_some()
                }
                return false;
            })
            .done();
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
