tool
extends Node

var TTS

var tts


func _ready():
	if OS.get_name() == "Server" or OS.has_feature("JavaScript"):
		return
	elif Engine.has_singleton("AndroidTTS"):
		tts = Engine.get_singleton("AndroidTTS")
	else:
		TTS = preload("godot-tts.gdns")
	if TTS and (TTS.can_instance() or Engine.editor_hint):
		tts = TTS.new()
	else:
		print_debug("TTS not available!")


func _get_min_rate():
	if OS.has_feature('JavaScript'):
		return 0.1
	elif Engine.has_singleton("AndroidTTS"):
		return 0.1
	elif tts != null:
		return tts.min_rate
	else:
		return 0


var min_rate setget , _get_min_rate


func _get_max_rate():
	if OS.has_feature('JavaScript'):
		return 10
	elif Engine.has_singleton("AndroidTTS"):
		return 10.0
	elif tts != null:
		return tts.max_rate
	else:
		return 0


var max_rate setget , _get_max_rate


func _get_normal_rate():
	if OS.has_feature('JavaScript'):
		return 1
	elif Engine.has_singleton("AndroidTTS"):
		return 1.0
	elif tts != null:
		return tts.normal_rate
	else:
		return 0


var normal_rate setget , _get_normal_rate

var javascript_rate = 50


func _set_rate(rate):
	if rate < self.min_rate:
		rate = self.min_rate
	elif rate > self.max_rate:
		rate = self.max_rate
	if Engine.has_singleton("AndroidTTS"):
		return tts.set_rate(rate)
	elif tts != null:
		tts.rate = rate
	elif OS.has_feature('JavaScript'):
		javascript_rate = rate


func _get_rate():
	if Engine.has_singleton("AndroidTTS"):
		return tts.get_rate()
	elif tts != null:
		return tts.rate
	elif OS.has_feature('JavaScript'):
		return javascript_rate
	else:
		return 0


var rate setget _set_rate, _get_rate


func _get_rate_percentage():
	return range_lerp(self.rate, self.min_rate, self.max_rate, 0, 100)


func _set_rate_percentage(v):
	self.rate = range_lerp(v, 0, 100, self.min_rate, self.max_rate)


var rate_percentage setget _set_rate_percentage, _get_rate_percentage


func _get_normal_rate_percentage():
	return range_lerp(self.normal_rate, self.min_rate, self.max_rate, 0, 100)


var normal_rate_percentage setget , _get_rate_percentage


func speak(text, interrupt := true):
	if tts != null:
		tts.speak(text, interrupt)
	elif OS.has_feature('JavaScript'):
		var code = (
			"""
            let utterance = new SpeechSynthesisUtterance("%s")
            utterance.rate = %s
        """
			% [text.replace("\n", " "), javascript_rate]
		)
		if interrupt:
			code += """
                window.speechSynthesis.cancel()
            """
		code += "window.speechSynthesis.speak(utterance)"
		JavaScript.eval(code)
	else:
		print_debug("%s: %s" % [text, interrupt])


func stop():
	if tts != null:
		tts.stop()
	elif OS.has_feature('JavaScript'):
		JavaScript.eval("window.speechSynthesis.cancel()")


func get_is_rate_supported():
	if Engine.has_singleton("AndroidTTS"):
		return true
	elif OS.has_feature('JavaScript'):
		return true
	elif tts != null:
		return tts.is_rate_supported()
	else:
		return false


var is_rate_supported setget , get_is_rate_supported


func _get_can_detect_screen_reader():
	if Engine.has_singleton("AndroidTTS"):
		return true
	elif OS.has_feature('JavaScript'):
		return false
	elif tts != null:
		return tts.can_detect_screen_reader
	return false


var can_detect_screen_reader setget , _get_can_detect_screen_reader


func _get_has_screen_reader():
	if Engine.has_singleton("AndroidTTS"):
		return tts.has_screen_reader()
	elif OS.has_feature('JavaScript'):
		return false
	elif tts != null:
		return tts.has_screen_reader
	return false


var has_screen_reader setget , _get_has_screen_reader


func singular_or_plural(count, singular, plural):
	if count == 1:
		return singular
	else:
		return plural


func _exit_tree():
	if not tts or not TTS:
		return
	tts.free()
