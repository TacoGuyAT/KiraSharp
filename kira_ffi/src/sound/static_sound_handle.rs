use std::ffi::c_void;
use std::mem;
use kira::sound::static_sound::StaticSoundHandle;
use kira::tween::Tween;
use kira::Volume;

#[no_mangle]
pub unsafe extern "C" fn destroy_static_sound_handle(static_sound_handle_ptr: *mut c_void) {
    let static_sound_handle = Box::from_raw(static_sound_handle_ptr as *mut StaticSoundHandle);
    drop(static_sound_handle);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_handle_resume(static_sound_handle_ptr: *mut c_void, tween_ptr: *mut c_void) {
    let mut static_sound_handle = Box::from_raw(static_sound_handle_ptr as *mut StaticSoundHandle);
    let tween = *Box::from_raw(tween_ptr as *mut Tween);
    static_sound_handle.resume(tween);
    mem::forget(static_sound_handle);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_handle_pause(static_sound_handle_ptr: *mut c_void, tween_ptr: *mut c_void) {
    let mut static_sound_handle = Box::from_raw(static_sound_handle_ptr as *mut StaticSoundHandle);
    let tween = *Box::from_raw(tween_ptr as *mut Tween);
    static_sound_handle.pause(tween);
    mem::forget(static_sound_handle);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_handle_stop(static_sound_handle_ptr: *mut c_void, tween_ptr: *mut c_void) {
    let mut static_sound_handle = Box::from_raw(static_sound_handle_ptr as *mut StaticSoundHandle);
    let tween = *Box::from_raw(tween_ptr as *mut Tween);
    static_sound_handle.stop(tween);
    mem::forget(static_sound_handle);
}

#[no_mangle]
pub unsafe extern "C" fn static_sound_handle_set_volume(static_sound_handle_ptr: *mut c_void, volume_ptr: *mut c_void, tween_ptr: *mut c_void) {
    let mut static_sound_handle = Box::from_raw(static_sound_handle_ptr as *mut StaticSoundHandle);
    let volume = *Box::from_raw(volume_ptr as *mut Volume);
    let tween = *Box::from_raw(tween_ptr as *mut Tween);
    static_sound_handle.set_volume(volume, tween);
    mem::forget(static_sound_handle);
}