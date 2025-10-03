use kira_ffi::*;
use std::error::Error;
use std::io::stdin;
use kira_ffi::sound::*;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { kira_ffi()? }
    stdin().read_line(&mut "".into())?;
    Ok(())
}

unsafe fn kira_ffi() -> Result<(), Box<dyn Error>> {
    let manager = create_audio_manager();
    let sound_data = create_static_sound_data_from_path(std::ffi::CString::new("C:\\flp\\akob\\akob-roulette4.wav")?.into_raw());
    static_sound_data_loop(sound_data);
    // static_sound_data_volume(sound_data, volume_amp(0.4));
    let track_builder = create_track_builder(1.0);
    track_builder_volume(track_builder, 0.1);
    let reverb = create_reverb_builder_default();
    reverb_builder_mix(reverb, 0.1);
    reverb_builder_damping(reverb, 0.01);
    reverb_builder_feedback(reverb, 0.92);
    let reverb_handle = track_builder_add_effect(track_builder, reverb_build(reverb));
    let track_handle = audio_manager_add_sub_track(manager, track_builder);
    static_sound_data_output_destination_track_handle(sound_data, track_handle);
    static_sound_data_play(manager, sound_data);
    Ok(())
}