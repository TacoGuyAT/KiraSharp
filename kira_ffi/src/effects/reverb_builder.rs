use crate::CEffect;
use kira::effect::reverb::{ReverbBuilder, ReverbHandle};
use kira::effect::EffectBuilder;
use std::ffi::c_void;
use std::mem;

#[no_mangle]
pub unsafe extern "C" fn create_reverb_builder(feedback: f64, damping: f64, stereo_width: f64, mix: f64) -> *mut c_void {
    let reverb_builder = ReverbBuilder::default()
        .feedback(feedback)
        .damping(damping)
        .stereo_width(stereo_width)
        .mix(mix);
    Box::into_raw(Box::new(reverb_builder)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn create_reverb_builder_default() -> *mut c_void {
    Box::into_raw(Box::new(ReverbBuilder::default())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn destroy_reverb_builder(reverb_handle_pointer: *mut c_void) {
    let reverb_handle = Box::from_raw(reverb_handle_pointer as *mut ReverbHandle);
    drop(reverb_handle);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_builder_feedback(reverb_pointer: *mut c_void, feedback: f64) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbBuilder);
    *reverb = reverb.feedback(feedback);
    mem::forget(reverb);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_builder_damping(reverb_pointer: *mut c_void, damping: f64) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbBuilder);
    *reverb = reverb.damping(damping);
    mem::forget(reverb);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_builder_stereo_width(reverb_pointer: *mut c_void, stereo_width: f64) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbBuilder);
    *reverb = reverb.stereo_width(stereo_width);
    mem::forget(reverb);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_builder_mix(reverb_pointer: *mut c_void, mix: f64) {
    let mut reverb = Box::from_raw(reverb_pointer as *mut ReverbBuilder);
    *reverb = reverb.mix(mix);
    mem::forget(reverb);
}

#[no_mangle]
pub unsafe extern "C" fn reverb_build(reverb_pointer: *mut c_void) -> *mut c_void {
    let reverb = Box::from_raw(reverb_pointer as *mut ReverbBuilder);
    let (effect, handle) = reverb.build();
    Box::into_raw(Box::new(CEffect::new(
        effect,
        Box::into_raw(Box::new(handle)) as *mut c_void
    ))) as *mut c_void
}