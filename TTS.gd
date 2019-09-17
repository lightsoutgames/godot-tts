extends Object

const TTS = preload("godot-tts.gdns")
var tts

func _init():
    tts = TTS.new()

func speak(text, interrupt := true):
    tts.speak(text, interrupt)

func stop():
    tts.stop()
