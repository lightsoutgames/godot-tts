# Load the Android build rules
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")

http_archive(
    name = "build_bazel_rules_android",
    sha256 = "cd06d15dd8bb59926e4d65f9003bfc20f9da4b2519985c27e190cddc8b7a7806",
    strip_prefix = "rules_android-0.1.1",
    urls = ["https://github.com/bazelbuild/rules_android/archive/v0.1.1.zip"],
)

# Configure Android SDK Path
load("@build_bazel_rules_android//android:rules.bzl", "aar_import", "android_library", "android_sdk_repository")

android_sdk_repository(name = "androidsdk")

http_file(
    name = "godot_lib",
    sha256 = "20c87d8aff967541985efd42ebe11f8c2c336b3a7681b8bbf714707cd7399639",
    urls = ["https://downloads.tuxfamily.org/godotengine/3.2.2/godot-lib.3.2.2.stable.release.aar"],
)
