tool
extends Node

const TTS = preload("godot-tts.gdns")

var tts = null

func _ready():
    # Only initialize TTS if it's available or if we're in the editor.
    if TTS.can_instance() or Engine.editor_hint:
        print("Attempting to load TTS.")
        tts = TTS.new()
    else:
        print("TTS not available!")

func set_rate(rate):
    if tts != null:
        tts.rate = rate

func get_rate():
    if tts != null:
        return tts.rate
    else:
        return 0

var rate setget set_rate, get_rate

func speak(text, interrupt := true):
    if tts != null:
        tts.speak(text, interrupt)

func stop():
    if tts != null:
        tts.stop()

func get_is_rate_supported():
    if tts != null:
        return tts.is_rate_supported()
    else:
        return false

var is_rate_supported setget , get_is_rate_supported

func singular_or_plural(count, singular, plural):
    if count == 1:
        return singular
    else:
        return plural
