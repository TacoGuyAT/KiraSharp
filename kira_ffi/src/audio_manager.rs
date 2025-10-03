use kira::manager::{AudioManager, AudioManagerSettings, DefaultBackend};
use kira::track::{TrackBuilder, TrackHandle};
use std::ffi::c_void;
use std::mem;
use kira::Volume;

#[no_mangle]
pub extern "C" fn create_audio_manager() -> *mut c_void {
    Box::into_raw(Box::new(
        AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Failed to create AudioManager")
    )) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn destroy_audio_manager(audio_manager_ptr: *mut c_void) {
    let audio_manager = Box::from_raw(audio_manager_ptr as *mut AudioManager);
    drop(audio_manager);
}

#[no_mangle]
pub unsafe extern "C" fn audio_manager_set_volume(audio_manager_ptr: *mut c_void, volume: *mut c_void, tween: *mut c_void) {
    let mut audio_manager = Box::from_raw(audio_manager_ptr as *mut AudioManager);
    let volume = *Box::from_raw(volume as *mut Volume);
    let tween = *Box::from_raw(tween as *mut kira::tween::Tween);
    audio_manager.main_track().set_volume(volume, tween);
    mem::forget(audio_manager);
}

#[no_mangle]
pub unsafe extern "C" fn audio_manager_add_sub_track(audio_manager_ptr: *mut c_void, builder: *mut c_void) -> *mut c_void {
    let mut audio_manager = Box::from_raw(audio_manager_ptr as *mut AudioManager);
    let builder = *Box::from_raw(builder as *mut TrackBuilder);
    let track = audio_manager.add_sub_track(builder).expect("Failed to add sub track");
    mem::forget(audio_manager);
    Box::into_raw(Box::new(track)) as *mut c_void
}