package games.lightsout.godot.tts;

import android.app.Activity;
import android.content.Intent;
import android.content.Context;
import android.speech.tts.TextToSpeech;
import android.util.Log;
import com.godot.game.R;
import javax.microedition.khronos.opengles.GL10;
import org.godotengine.godot.Godot;

public class TTS extends Godot.SingletonBase implements TextToSpeech.OnInitListener {

    protected Activity appActivity;
    protected Context appContext;
    private Godot activity = null;
    private int instanceId = 0;

    private TextToSpeech tts = null;

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

    public void set_rate(Float rate) {
        Float newRate;
        if (rate <= 50)
            newRate = rate / 50;
        else {
            newRate = rate - 50;
            newRate = 1 + (newRate / 5);
        }
        tts.setSpeechRate(newRate);
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
        registerClass("AndroidTTS", new String[] { "speak", "stop", "set_rate", "getInstanceId" });
        this.activity.runOnUiThread(new Runnable() {
            public void run() {
            }
        });

    }

    public void onInit(int status) {
    }

}
