tool
extends Node

const TTS = preload("godot-tts.gdns")

var tts = TTS.new()

func set_rate(rate):
    tts.rate = rate

func get_rate():
    return tts.rate

var rate setget set_rate, get_rate

func speak(text, interrupt := true):
    tts.speak(text, interrupt)

func stop():
    tts.stop()

func get_is_rate_supported():
    return tts.is_rate_supported()

var is_rate_supported setget , get_is_rate_supported

func singular_or_plural(count, singular, plural):
    if count == 1:
        return singular
    else:
        return plural
