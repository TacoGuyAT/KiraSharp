pub mod audio_manager;
pub mod track_builder;
pub mod effects;
pub mod track_handle;
pub mod tween;
pub mod sound;

use std::ffi::c_void;
use std::time::Duration;
use kira::{StartTime, Volume};
use kira::tween::Easing;
pub use audio_manager::*;
pub use track_builder::*;
pub use effects::*;

#[no_mangle]
pub unsafe extern "C" fn start_time(start_time: f64) -> *mut c_void {
    Box::into_raw(Box::new(StartTime::Delayed(Duration::from_secs_f64(start_time)))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn start_time_immediate() -> *mut c_void {
    Box::into_raw(Box::new(StartTime::Immediate)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn duration(duration_ms: f64) -> *mut c_void {
    Box::into_raw(Box::new(Duration::from_micros((duration_ms * 1000.) as u64))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn easing_linear() -> *mut c_void {
    Box::into_raw(Box::new(Easing::Linear)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn easing(ease_in: bool, ease_out: bool, value: i32) -> *mut c_void {
    let easing = match (ease_in, ease_out) {
        (false, false) => Easing::Linear,
        (true, false) => Easing::InPowi(value),
        (false, true) => Easing::OutPowi(value),
        (true, true) => Easing::InOutPowi(value),
    };
    Box::into_raw(Box::new(easing)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn easing_f(ease_in: bool, ease_out: bool, value: f64) -> *mut c_void {
    let easing = match (ease_in, ease_out) {
        (false, false) => Easing::Linear,
        (true, false) => Easing::InPowf(value),
        (false, true) => Easing::OutPowf(value),
        (true, true) => Easing::InOutPowf(value),
    };
    Box::into_raw(Box::new(easing)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn volume_amp(volume: f64) -> *mut c_void {
    Box::into_raw(Box::new(Volume::Amplitude(volume))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn volume_db(volume: f64) -> *mut c_void {
    Box::into_raw(Box::new(Volume::Decibels(volume))) as *mut c_void
}