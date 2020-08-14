load("@build_bazel_rules_android//android:rules.bzl", "aar_import", "android_library")

aar_import(
    name = "godot",
    aar = "@godot_lib",
)

android_library(
    name = "godot_tts",
    srcs = ["TTS.java"],
    manifest = "AndroidManifest.xml",
    deps = [":godot"],
)
