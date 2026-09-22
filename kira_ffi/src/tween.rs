use std::ffi::c_void;
use std::mem;
use std::time::Duration;
use kira::StartTime;
use kira::tween::{Easing, Tween};

#[no_mangle]
pub unsafe extern "C" fn create_tween(start_time: *mut c_void, duration: *mut c_void, easing: *mut c_void) -> *mut c_void {
    let tween = Tween {
        start_time: *Box::from_raw(start_time as *mut StartTime),
        duration: *Box::from_raw(duration as *mut Duration),
        easing: *Box::from_raw(easing as *mut Easing),
    };
    Box::into_raw(Box::new(tween)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn create_tween_default() -> *mut c_void {
    Box::into_raw(Box::new(Tween::default())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn destroy_tween(tween_ptr: *mut c_void) {
    let tween = Box::from_raw(tween_ptr as *mut Tween);
    drop(tween);
}

#[no_mangle]
pub unsafe extern "C" fn tween_start_time(tween_ptr: *mut c_void, start_time_ptr: *mut c_void) {
    let mut tween = Box::from_raw(tween_ptr as *mut Tween);
    let start_time = Box::from_raw(start_time_ptr as *mut StartTime);
    tween.start_time = *start_time;
    mem::forget(tween);
}

#[no_mangle]
pub unsafe extern "C" fn tween_duration(tween_ptr: *mut c_void, duration_ptr: *mut c_void) {
    let mut tween = Box::from_raw(tween_ptr as *mut Tween);
    let duration = Box::from_raw(duration_ptr as *mut Duration);
    tween.duration = *duration;
    mem::forget(tween);
}

#[no_mangle]
pub unsafe extern "C" fn tween_easing(tween_ptr: *mut c_void, easing_ptr: *mut c_void) {
    let mut tween = Box::from_raw(tween_ptr as *mut Tween);
    let easing = *Box::from_raw(easing_ptr as *mut Easing);
    tween.easing = easing;
    mem::forget(tween);
}