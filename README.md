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

Note that not all features are supported on all synthesizers. Any feature that may not be supported has a boolean property that can be checked to determine whether it works on the current platform.

* Speaking text, with interruption support on most platforms
* Stopping speech in progress
* Getting and setting speech rate in both native synthesizer units and percentages
* Detecting whether a screen reader is active
* Determining whether the synthesizer is speaking, and sending a signal on completion

## API

The public-facing API is contained entirely within [TTS.GD](https://github.com/lightsoutgames/godot-tts/blob/master/TTS.gd).

## Download

Download the [latest release](https://github.com/lightsoutgames/godot-tts/releases).

## Notes on Universal Windows Platform

Godot's UWP export is [currently broken](https://github.com/godotengine/godot/issues/30558). In order to use this addon in a UWP game, do the following:

1. [Export to UWP](https://docs.godotengine.org/en/stable/getting_started/workflow/export/exporting_for_uwp.html).
2. [Extract the newly-created .appx file with `makeappx unpack`](https://docs.microsoft.com/en-us/windows/msix/package/create-app-package-with-makeappx-tool).
3. If you've extracted the .appx file to a directory like _mygame\_, copy _addons\godot-tts\target\release\godot_tts.dll_ to _mygame\game\addons\godot-tts\target\release\godot_tts.dll_, creating the directory if it doesn't exist.
4. Repack the appx using _makeappx_.

It should then be ready to sign and install. Hopefully Godot's UWP export will eventually copy GDNative DLLs correctly so this procedure isn't necessary.
