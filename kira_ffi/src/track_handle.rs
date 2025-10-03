use std::ffi::c_void;
use std::mem;
use kira::track::TrackHandle;
use kira::tween::Tween;
use kira::Volume;

#[no_mangle]
pub unsafe extern "C" fn destroy_track_handle(track_handle_ptr: *mut c_void) {
    let track_handle = Box::from_raw(track_handle_ptr as *mut TrackHandle);
    drop(track_handle);
}

#[no_mangle]
pub unsafe extern "C" fn track_set_volume(track_handle_ptr: *mut c_void, volume_ptr: *mut c_void, tween_ptr: *mut c_void) {
    let mut track_handle = Box::from_raw(track_handle_ptr as *mut TrackHandle);
    let volume = *Box::from_raw(volume_ptr as *mut Volume);
    let tween = *Box::from_raw(tween_ptr as *mut Tween);
    track_handle.set_volume(volume, tween);
    mem::forget(track_handle);
}

// #[no_mangle]
// pub unsafe extern "C" fn track_handle_set_route(track_handle_ptr: *mut c_void, volume: f64, tween_ptr: *mut c_void) {
//     let mut track_handle = Box::from_raw(track_handle_ptr as *mut TrackHandle);
//     let mut tween = Box::from_raw(tween_ptr as *mut Tween);
//     track_handle.set_route(volume.into(), *tween);
//     mem::forget(track_handle);
// }