package games.lightsout.godot.tts;

import java.util.Arrays;
import java.util.List;

import org.godotengine.godot.Godot;
import org.godotengine.godot.plugin.GodotPlugin;

import android.accessibilityservice.AccessibilityServiceInfo;
import android.app.Activity;
import android.content.Context;
import android.speech.tts.TextToSpeech;
import android.view.accessibility.AccessibilityManager;

public class TTS extends GodotPlugin implements TextToSpeech.OnInitListener {
    private TextToSpeech tts = null;

    private float rate = 1f;

    private Integer utteranceId = 0;

    public void speak(String text, boolean interrupt) {
        int mode = TextToSpeech.QUEUE_ADD;
        if (interrupt)
            mode = TextToSpeech.QUEUE_FLUSH;
        tts.speak(text, mode, null, this.utteranceId.toString());
        this.utteranceId++;
    }

    public void stop() {
        tts.stop();
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

    public TTS(Godot godot) {
        super(godot);
        this.tts = new TextToSpeech(this.getActivity(), this);
    }

    public String getPluginName() {
        return "GodotTTS";
    }

    public List<String> getPluginMethods() {
        return Arrays.asList("speak", "stop", "get_rate", "set_rate", "has_screen_reader", "is_speaking");
    }

    public void onInit(int status) {
    }

}
