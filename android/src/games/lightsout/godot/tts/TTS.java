package games.lightsout.godot.tts;

import java.util.List;

import android.accessibilityservice.AccessibilityServiceInfo;
import android.app.Activity;
import android.content.Intent;
import android.content.Context;
import android.speech.tts.TextToSpeech;
import android.util.Log;
import android.view.accessibility.AccessibilityManager;

import com.godot.game.R;
import javax.microedition.khronos.opengles.GL10;
import org.godotengine.godot.Godot;

public class TTS extends Godot.SingletonBase implements TextToSpeech.OnInitListener {

    protected Activity appActivity;
    protected Context appContext;
    private Godot activity = null;
    private int instanceId = 0;

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

    public boolean has_screen_reader() {
        AccessibilityManager accessibilityManager = (AccessibilityManager) appContext
                .getSystemService(Context.ACCESSIBILITY_SERVICE);
        if (accessibilityManager != null) {
            List<AccessibilityServiceInfo> screenReaders =accessibilityManager.getEnabledAccessibilityServiceList(AccessibilityServiceInfo.FEEDBACK_SPOKEN);
            return screenReaders.size() != 0;
        } else
            return false;
    }

    public void getInstanceId(int pInstanceId) {
        // You will need to call this method from Godot and pass in the
        // get_instance_id().
        instanceId = pInstanceId;
    }

    static public Godot.SingletonBase initialize(Activity p_activity) {
        return new TTS(p_activity);
    }

    public TTS(Activity p_activity) {
        this.activity = (Godot) p_activity;
        this.appActivity = p_activity;
        this.appContext = appActivity.getApplicationContext();
        this.tts = new TextToSpeech(this.appContext, this);
        // Register class name and functions to bind.
        registerClass("AndroidTTS", new String[] { "speak", "stop", "get_rate", "set_rate", "getInstanceId" });
        this.activity.runOnUiThread(new Runnable() {
            public void run() {
            }
        });

    }

    public void onInit(int status) {
    }

}
