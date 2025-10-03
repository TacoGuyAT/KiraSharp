use kira::sound::static_sound::StaticSoundData;
use kira::sound::IntoOptionalRegion;
use kira::track::TrackHandle;
use std::ffi::{c_char, c_void, CStr};
use std::io::Cursor;
use std::mem;
use std::ops::Deref;
use std::path::Path;
use kira::manager::AudioManager;
use kira::Volume;

#[no_mangle]
pub unsafe extern "C" fn create_static_sound_data_from_path(path_ptr: *mut c_char) -> *mut c_void {
    let path = CStr::from_ptr(path_ptr).to_str().expect("Couldn't convert CString to &str");
    Box::into_raw(Box::new(StaticSoundData::from_file(Path::new(path)).expect(&format!("Invalid path (\"{path}\")")))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn create_static_sound_data_from_bytes(data: *mut u8, len: usize) -> *mut c_void {
    Box::into_raw(Box::new(StaticSoundData::from_cursor(Cursor::new(std::slice::from_raw_parts(data, len))).expect("Failed to create sound data"))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn destroy_static_sound_data(sound_data_ptr: *mut c_void) {
    let sound_data = Box::from_raw(sound_data_ptr as *mut StaticSoundData);
    drop(sound_data);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_data_output_destination_track_handle(sound_data_ptr: *mut c_void, track_handle_ptr: *mut c_void) {
    let mut sound_data = Box::from_raw(sound_data_ptr as *mut StaticSoundData);
    let destination = &mut *(track_handle_ptr as *mut TrackHandle);
    sound_data.settings.output_destination = destination.deref().into();
    mem::forget(sound_data);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_data_volume(sound_data_ptr: *mut c_void, volume_ptr: *mut c_void) {
    let mut sound_data = Box::from_raw(sound_data_ptr as *mut StaticSoundData);
    let volume = *Box::from_raw(volume_ptr as *mut Volume);
    sound_data.settings.volume = volume.into();
    mem::forget(sound_data);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_data_loop(sound_data_ptr: *mut c_void) {
    let mut sound_data = Box::from_raw(sound_data_ptr as *mut StaticSoundData);
    let a = ..;
    sound_data.settings.loop_region = (..).into_optional_region();
    mem::forget(sound_data);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_data_loop_bounded(sound_data_ptr: *mut c_void, start: f64, end: f64) {
    let mut sound_data = Box::from_raw(sound_data_ptr as *mut StaticSoundData);
    sound_data.settings.loop_region = (start..end).into_optional_region();
    mem::forget(sound_data);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_data_panning(sound_data_ptr: *mut c_void, panning: f64) {
    let mut sound_data = Box::from_raw(sound_data_ptr as *mut StaticSoundData);
    sound_data.settings.panning = panning.into();
    mem::forget(sound_data);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_data_play(audio_manager_ptr: *mut c_void, sound_data_ptr: *mut c_void) -> *mut c_void {
    let mut audio_manager = Box::from_raw(audio_manager_ptr as *mut AudioManager);
    let sound_data = *Box::from_raw(sound_data_ptr as *mut StaticSoundData);
    let handle =  audio_manager.play(sound_data).expect("Failed to play sound");
    mem::forget(audio_manager);
    Box::into_raw(Box::new(handle)) as *mut c_void
}