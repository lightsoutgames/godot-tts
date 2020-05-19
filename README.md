# Godot TTS

This addon was primarily developed for the [Godot Accessibility addon](https://github.com/lightsoutgames/godot-accessibility), but should work well in other contexts where a Godot game might require text-to-speech.

## Supported Platforms

Text-to-speech is complicated, and some features may not work everywhere. Most optional features have an associated boolean property used to determine if the feature is available. Further, while I do attempt to ensure that this addon works as well as possible on all platforms, there may be bugs, and pull requests are welcome. Known supported platforms include:

* Windows
  * Screen readers via [Tolk](https://github.com/dkager/tolk/)
  * Native WinRT
* Linux via [Speech Dispatcher](https://freebsoft.org/speechd)
* HTML 5
* Android

## Features

* Speaking text, with interruption support on most platforms
* Stopping speech in progress
* Getting and setting speech rate in both native synthesizer units and percentages
* Detecting whether a screen reader is active

## API

The public-facing API is contained entirely within [TTS.GD](https://github.com/lightsoutgames/godot-tts/blob/master/TTS.gd).

## Download

Download the [latest release](https://github.com/lightsoutgames/godot-tts/releases).