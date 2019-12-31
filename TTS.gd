tool
extends Node

var TTS

var tts = null

func _ready():
    if not OS.has_feature('JavaScript') and not Engine.has_singleton("AndroidTTS"):
        TTS  = preload("godot-tts.gdns")
    if OS.has_feature('JavaScript'):
        pass
    elif Engine.has_singleton("AndroidTTS"):
        tts = Engine.get_singleton("AndroidTTS")
    elif TTS.can_instance() or Engine.editor_hint:
        tts = TTS.new()
    else:
        print_debug("TTS not available!")

var javascript_rate = 50

func set_rate(rate):
    if tts != null:
        tts.rate = rate
    elif OS.has_feature('JavaScript'):
        javascript_rate = rate

func get_rate():
    if tts != null:
        return tts.rate
    elif OS.has_feature('JavaScript'):
        return javascript_rate
    else:
        return 0

var rate setget set_rate, get_rate

func speak(text, interrupt := true):
    print_debug("%s: %s" % [text, interrupt])
    if tts != null:
        tts.speak(text, interrupt)
    elif OS.has_feature('JavaScript'):
        var scaled_rate = javascript_rate / 2
        var code = """
            let utterance = new SpeechSynthesisUtterance("%s")
            utterance.rate = %s
        """ % [text, scaled_rate]
        if interrupt:
            code += """
                window.speechSynthesis.cancel()
            """
        code += "window.speechSynthesis.speak(utterance)"
        JavaScript.eval(code)

func stop():
    if tts != null:
        tts.stop()
    elif OS.has_feature('JavaScript'):
        JavaScript.eval("window.speechSynthesis.cancel()")

func get_is_rate_supported():
    if Engine.get_singleton("AndroidTTS"):
        return false
    elif OS.has_feature('JavaScript'):
        return true
    elif tts != null:
        return tts.is_rate_supported()
    else:
        return false

var is_rate_supported setget , get_is_rate_supported

func singular_or_plural(count, singular, plural):
    if count == 1:
        return singular
    else:
        return plural

func _exit_tree():
    tts.free()
