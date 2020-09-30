use std::sync::mpsc::{channel, Receiver};

use gdnative::prelude::*;
use tts::{Features, UtteranceId, TTS as Tts};

#[derive(NativeClass)]
#[inherit(Reference)]
struct Utterance(pub(crate) Option<UtteranceId>);

#[methods]
impl Utterance {
    fn new(_owner: &Reference) -> Self {
        Self(None)
    }
}

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
    fn new(owner: &Node) -> Self {
        owner.set_pause_mode(2);
        let tts = Tts::default().unwrap();
        let (tx, rx) = channel();
        let Features {
            utterance_callbacks,
            ..
        } = tts.supported_features();
        if utterance_callbacks {
            let tx_end = tx.clone();
            tts.on_utterance_begin(Some(Box::new(move |utterance| {
                tx.send(Msg::UtteranceBegin(utterance)).unwrap();
            })))
            .expect("Failed to set utterance_begin callback");
            tts.on_utterance_end(Some(Box::new(move |utterance| {
                tx_end.send(Msg::UtteranceEnd(utterance)).unwrap();
            })))
            .expect("Failed to set utterance_end callback");
        }
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
            args: &[SignalArgument {
                name: "utterance",
                default: Variant::default(),
                export_info: ExportInfo::new(VariantType::Object),
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "utterance_end",
            args: &[SignalArgument {
                name: "utterance",
                default: Variant::default(),
                export_info: ExportInfo::new(VariantType::Object),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    #[export]
    fn speak(&mut self, _owner: &Node, message: GodotString, interrupt: bool) -> Variant {
        let message = message.to_string();
        if let Ok(id) = self.0.speak(message, interrupt) {
            let utterance: Instance<Utterance, Unique> = Instance::new();
            if id.is_some() {
                utterance
                    .map_mut(|u, _| u.0 = id)
                    .expect("Failed to set utterance ID");
            }
            let utterance = utterance.owned_to_variant();
            utterance
        } else {
            Variant::default()
        }
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
    fn are_utterance_callbacks_supported(&mut self, _owner: &Node) -> bool {
        let Features {
            utterance_callbacks: supported,
            ..
        } = self.0.supported_features();
        supported
    }

    #[export]
    fn _process(&mut self, owner: &Node, _delta: f32) {
        if let Some(msg) = self.1.try_recv().ok() {
            match msg {
                Msg::UtteranceBegin(utterance_id) => {
                    let utterance: Instance<Utterance, Unique> = Instance::new();
                    utterance
                        .map_mut(|u, _| u.0 = Some(utterance_id))
                        .expect("Failed to set utterance ID");
                    owner.emit_signal("utterance_begin", &[utterance.owned_to_variant()]);
                }
                Msg::UtteranceEnd(utterance_id) => {
                    let utterance: Instance<Utterance, Unique> = Instance::new();
                    utterance
                        .map_mut(|u, _| u.0 = Some(utterance_id))
                        .expect("Failed to set utterance ID");
                    owner.emit_signal("utterance_end", &[utterance.owned_to_variant()]);
                }
            }
        }
    }
}

fn init(handle: InitHandle) {
    env_logger::init();
    handle.add_class::<Utterance>();
    handle.add_class::<TTS>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
