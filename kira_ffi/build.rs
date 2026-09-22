use std::env;

fn main() {
    #[cfg(all(feature = "csbindgen"))]
    csbindgen();
}

#[cfg(all(feature = "csbindgen"))]
fn csbindgen() {
    // Path to C# output file
    let path = env::var("KIRA_FFI_CSBINDGEN_PATH")
        // .unwrap();
        .unwrap_or(format!("{}/target/Generated/FFI.g.cs", env::var("CARGO_MANIFEST_DIR").unwrap()));
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/audio_manager.rs")
        .input_extern_file("src/track_builder.rs")
        .input_extern_file("src/track_handle.rs")
        .input_extern_file("src/tween.rs")
        .input_extern_file("src/sound/static_sound_data.rs")
        .input_extern_file("src/sound/static_sound_handle.rs")
        .input_extern_file("src/sound/streaming_sound_data.rs")
        .input_extern_file("src/sound/pcm_stream.rs")
        .input_extern_file("src/effects/mod.rs")
        .input_extern_file("src/effects/managed_effect.rs")
        .input_extern_file("src/effects/reverb_builder.rs")
        .input_extern_file("src/effects/reverb_handle.rs")
        .csharp_dll_name("kira_ffi")
        .csharp_namespace("KiraSharp.Generated")
        .csharp_class_name("FFI")
        .generate_csharp_file(path)
        .unwrap();
}