use crate::CEffect;
use kira::track::TrackBuilder;
use std::ffi::c_void;
use std::mem;

#[no_mangle]
pub unsafe extern "C" fn create_track_builder(volume: f64) -> *mut c_void {
    Box::into_raw(Box::new(TrackBuilder::new().volume(volume))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn create_track_builder_default() -> *mut c_void {
    Box::into_raw(Box::new(TrackBuilder::new())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn destroy_track_builder(builder_ptr: *mut c_void) {
    let builder = Box::from_raw(builder_ptr as *mut TrackBuilder);
    drop(builder);
}

#[no_mangle]
pub unsafe extern "C" fn track_builder_volume(builder_ptr: *mut c_void, volume: f64) {
    let mut builder = Box::from_raw(builder_ptr as *mut TrackBuilder);
    *builder = builder.volume(volume);
    mem::forget(builder);
}

#[no_mangle]
pub unsafe extern "C" fn track_builder_add_effect(builder_ptr: *mut c_void, ceffect: *mut c_void) -> *mut c_void {
    let mut builder = Box::from_raw(builder_ptr as *mut TrackBuilder);
    let effect = Box::from_raw(ceffect as *mut CEffect);
    let CEffect {
        effect,
        handle,
    } = *effect;
    builder.add_built_effect(effect);
    mem::forget(builder);
    handle
}