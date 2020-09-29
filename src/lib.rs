use std::sync::mpsc::{channel, Receiver};

use gdnative::prelude::*;
use tts::{Features, UtteranceId, TTS as Tts};

enum Msg {
    UtteranceBegin(UtteranceId),
    UtteranceEnd(UtteranceId),
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
struct TTS(Tts, Receiver<Msg>);

#[methods]
impl TTS {
    fn new(_owner: &Node) -> Self {
        let tts = Tts::default().unwrap();
        let (tx, rx) = channel();
        let tx_end = tx.clone();
        tts.on_utterance_begin(Some(Box::new(move |utterance| {
            tx.send(Msg::UtteranceBegin(utterance)).unwrap();
        })))
        .expect("Failed to set utterance_begin callback");
        tts.on_utterance_end(Some(Box::new(move |utterance| {
            tx_end.send(Msg::UtteranceEnd(utterance)).unwrap();
        })))
        .expect("Failed to set utterance_end callback");
        Self(tts, rx)
    }

    fn register(builder: &ClassBuilder<Self>) {
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
            .with_getter(|_: &TTS, _| if cfg!(windows) { true } else { false })
            .done();
        #[allow(unreachable_code)]
        builder
            .add_property("has_screen_reader")
            .with_getter(|_: &TTS, _| {
                #[cfg(windows)]
                {
                    let tolk = tolk::Tolk::new();
                    return tolk.detect_screen_reader().is_some();
                }
                false
            })
            .done();
        builder
            .add_property("can_detect_is_speaking")
            .with_getter(|this: &TTS, _| {
                let Features {
                    is_speaking: is_speaking_supported,
                    ..
                } = this.0.supported_features();
                return is_speaking_supported;
            })
            .done();
        builder
            .add_property("is_speaking")
            .with_getter(|this: &TTS, _| {
                let Features {
                    is_speaking: is_speaking_supported,
                    ..
                } = this.0.supported_features();
                if is_speaking_supported {
                    return this.0.is_speaking().unwrap();
                } else {
                    return false;
                }
            })
            .done();
        builder.add_signal(Signal {
            name: "utterance_begin",
            args: &[],
        });
        builder.add_signal(Signal {
            name: "utterance_end",
            args: &[],
        });
    }

    #[export]
    fn speak(&mut self, _owner: &Node, message: GodotString, interrupt: bool) {
        let message = message.to_string();
        self.0.speak(message, interrupt).unwrap();
    }

    #[export]
    fn stop(&mut self, _owner: &Node) {
        self.0.stop().unwrap();
    }

    #[export]
    fn is_rate_supported(&mut self, _owner: &Node) -> bool {
        let Features {
            rate: rate_supported,
            ..
        } = self.0.supported_features();
        rate_supported
    }

    #[export]
    fn _process(&mut self, owner: &Node, _delta: f32) {
        if let Some(msg) = self.1.try_recv().ok() {
            match msg {
                Msg::UtteranceBegin(_utterance) => {
                    owner.emit_signal("utterance_begin", &[]);
                }
                Msg::UtteranceEnd(_utterance) => {
                    owner.emit_signal("utterance_end", &[]);
                }
            }
        }
    }
}

fn init(handle: InitHandle) {
    env_logger::init();
    handle.add_class::<TTS>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
