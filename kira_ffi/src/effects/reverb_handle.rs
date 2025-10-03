use std::ffi::c_void;
use std::mem;
use kira::effect::reverb::ReverbHandle;
use kira::tween::Tween;

#[no_mangle]
pub unsafe extern "C" fn destroy_reverb_handle(reverb_handle_pointer: *mut c_void) {
    let reverb_handle = Box::from_raw(reverb_handle_pointer as *mut ReverbHandle);
    drop(reverb_handle);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_feedback(reverb_pointer: *mut c_void, feedback: f64, tween: *mut c_void) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbHandle);
    let tween = *Box::from_raw(tween as *mut Tween);
    reverb.set_feedback(feedback, tween);
    mem::forget(reverb);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_damping(reverb_pointer: *mut c_void, damping: f64, tween: *mut c_void) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbHandle);
    let tween = *Box::from_raw(tween as *mut Tween);
    reverb.set_damping(damping, tween);
    mem::forget(reverb);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_stereo_width(reverb_pointer: *mut c_void, stereo_width: f64, tween: *mut c_void) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbHandle);
    let tween = *Box::from_raw(tween as *mut Tween);
    reverb.set_stereo_width(stereo_width, tween);
    mem::forget(reverb);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_mix(reverb_pointer: *mut c_void, mix: f64, tween: *mut c_void) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbHandle);
    let tween = *Box::from_raw(tween as *mut Tween);
    reverb.set_mix(mix, tween);
    mem::forget(reverb);
}