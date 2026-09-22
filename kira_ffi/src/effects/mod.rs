use std::ffi::c_void;
use kira::effect::Effect;

pub mod managed_effect;
pub mod reverb_builder;
pub mod reverb_handle;

pub use managed_effect::*;
pub use reverb_builder::*;
pub use reverb_handle::*;

#[repr(C)]
pub struct CEffect {
    pub effect: Box<dyn Effect>,
    pub handle: *mut c_void,
    /// Destructor for `handle`. Stored here because the concrete handle type is
    /// erased to `*mut c_void`, so a generic `destroy_ceffect` cannot otherwise
    /// know how to drop it.
    pub destroy_handle: unsafe extern "C" fn(*mut c_void),
}

impl CEffect {
    pub fn new(
        effect: Box<dyn Effect>,
        handle: *mut c_void,
        destroy_handle: unsafe extern "C" fn(*mut c_void),
    ) -> Self {
        Self {
            effect,
            handle,
            destroy_handle,
        }
    }
}

/// Destroys a `CEffect` that was built (via `*_build`) but never attached to a
/// track. Frees both the boxed effect and its associated handle.
#[no_mangle]
pub unsafe extern "C" fn destroy_ceffect(ceffect_ptr: *mut c_void) {
    let ceffect = Box::from_raw(ceffect_ptr as *mut CEffect);
    (ceffect.destroy_handle)(ceffect.handle);
    drop(ceffect);
}