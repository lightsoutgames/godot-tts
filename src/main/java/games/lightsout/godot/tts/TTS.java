package games.lightsout.godot.tts;

import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

import org.godotengine.godot.Godot;
import org.godotengine.godot.plugin.GodotPlugin;
import org.godotengine.godot.plugin.SignalInfo;

import android.accessibilityservice.AccessibilityServiceInfo;
import android.content.Context;
import android.speech.tts.TextToSpeech;
import android.speech.tts.UtteranceProgressListener;
import android.view.accessibility.AccessibilityManager;

public class TTS extends GodotPlugin implements TextToSpeech.OnInitListener {
    private TextToSpeech tts = null;

    private float volume = 1f;
    private float rate = 1f;

    private Integer utteranceId = 0;

    public int speak(String text, boolean interrupt) {
        int mode = TextToSpeech.QUEUE_ADD;
        if (interrupt)
            mode = TextToSpeech.QUEUE_FLUSH;
        tts.speak(text, mode, null, this.utteranceId.toString());
        int rv = this.utteranceId.intValue();
        this.utteranceId++;
        return rv;
    }

    public void stop() {
        tts.stop();
    }

    public float get_volume() {
        return this.volume;
    }

    public void set_volume(float volume) {
        this.volume = volume;
        tts.Engine.KEY_PARAM_VOLUME = volume;
    }

    public float get_rate() {
        return this.rate;
    }

    public void set_rate(float rate) {
        this.rate = rate;
        tts.setSpeechRate(rate);
    }

    public boolean is_speaking() {
        return tts.isSpeaking();
    }

    public boolean has_screen_reader() {
        AccessibilityManager accessibilityManager = (AccessibilityManager) getActivity()
                .getSystemService(Context.ACCESSIBILITY_SERVICE);
        if (accessibilityManager != null) {
            List<AccessibilityServiceInfo> screenReaders = accessibilityManager
                    .getEnabledAccessibilityServiceList(AccessibilityServiceInfo.FEEDBACK_SPOKEN);
            return screenReaders.size() != 0;
        } else {
            return false;
        }
    }

    class Listener extends UtteranceProgressListener {
        public void onStart(String utteranceId) {
            Integer id = Integer.parseInt(utteranceId);
            TTS.this.emitSignal("utterance_begin", id);
        }

        public void onStop(String utteranceId, Boolean interrupted) {
            Integer id = Integer.parseInt(utteranceId);
            TTS.this.emitSignal("utterance_stop", id);
        }

        public void onDone(String utteranceId) {
            Integer id = Integer.parseInt(utteranceId);
            TTS.this.emitSignal("utterance_end", id);
        }

        public void onError(String utteranceId) {
            Integer id = Integer.parseInt(utteranceId);
            TTS.this.emitSignal("utterance_end", id);
        }
    }

    public TTS(Godot godot) {
        super(godot);
        this.tts = new TextToSpeech(this.getActivity(), this);
        tts.setOnUtteranceProgressListener(new Listener());
    }

    @Override
    public String getPluginName() {
        return "GodotTTS";
    }

    @Override
    public List<String> getPluginMethods() {
        return Arrays.asList("speak", "stop", "get_rate", "set_rate", "has_screen_reader", "is_speaking");
    }

    @Override
    public Set<SignalInfo> getPluginSignals() {
        SignalInfo begin = new SignalInfo("utterance_begin", Integer.class);
        SignalInfo end = new SignalInfo("utterance_end", Integer.class);
        SignalInfo stop = new SignalInfo("utterance_stop", Integer.class);
        return new HashSet(Arrays.asList(begin, end, stop));
    }

    public void onInit(int status) {
    }

}
