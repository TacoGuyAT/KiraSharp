use crate::CEffect;
use kira::clock::clock_info::ClockInfoProvider;
use kira::effect::Effect;
use kira::modulator::value_provider::ModulatorValueProvider;
use kira::Frame;
use std::ffi::c_void;

/// An effect whose DSP is implemented on the managed (C#) side.
///
/// `process` is a C# function pointer (an `[UnmanagedCallersOnly]` method). It
/// receives `user_data` (typically a `GCHandle` to the managed effect object)
/// and a pointer to a 2-element `[left, right]` f32 buffer that it reads and
/// writes in place. `init` is called once with the renderer's sample rate.
///
/// NOTE: `process` is invoked on kira's audio render thread, once per sample
/// frame. Crossing into managed code here is not strictly real-time-safe (a GC
/// pause could cause an audio glitch); it is acceptable for examples/tools but
/// production DSP should live in Rust.
pub struct ManagedEffect {
    process: unsafe extern "C" fn(user_data: *mut c_void, frame: *mut f32),
    init: unsafe extern "C" fn(user_data: *mut c_void, sample_rate: u32),
    user_data: *mut c_void,
}

// The raw pointers are an opaque managed handle plus a function pointer. kira
// moves the effect to the renderer thread and requires `Send + Sync`; the
// managed side is responsible for keeping `user_data` valid for the effect's
// lifetime.
unsafe impl Send for ManagedEffect {}
unsafe impl Sync for ManagedEffect {}

impl Effect for ManagedEffect {
    fn init(&mut self, sample_rate: u32) {
        // Called on the thread that builds the track (not the audio thread).
        unsafe { (self.init)(self.user_data, sample_rate) };
    }

    fn process(
        &mut self,
        input: Frame,
        _dt: f64,
        _clock_info_provider: &ClockInfoProvider,
        _modulator_value_provider: &ModulatorValueProvider,
    ) -> Frame {
        let mut buf = [input.left, input.right];
        unsafe { (self.process)(self.user_data, buf.as_mut_ptr()) };
        Frame {
            left: buf[0],
            right: buf[1],
        }
    }
}

/// No-op handle destructor: a managed effect has no kira-side handle to free (the
/// managed side owns its own state via `user_data`).
unsafe extern "C" fn destroy_managed_effect_handle(_handle: *mut c_void) {}

/// Builds a `CEffect` backed by a C# `process` callback. The returned pointer
/// can be attached to a track with `track_builder_add_effect`, or freed with
/// `destroy_ceffect` if it is never attached.
#[no_mangle]
pub unsafe extern "C" fn create_managed_effect(
    process: unsafe extern "C" fn(user_data: *mut c_void, frame: *mut f32),
    init: unsafe extern "C" fn(user_data: *mut c_void, sample_rate: u32),
    user_data: *mut c_void,
) -> *mut c_void {
    let effect: Box<dyn Effect> = Box::new(ManagedEffect { process, init, user_data });
    Box::into_raw(Box::new(CEffect::new(
        effect,
        std::ptr::null_mut(),
        destroy_managed_effect_handle,
    ))) as *mut c_void
}
