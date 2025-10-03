use std::ffi::c_void;
use kira::effect::Effect;

pub mod reverb_builder;
pub mod reverb_handle;

pub use reverb_builder::*;
pub use reverb_handle::*;

#[repr(C)]
pub struct CEffect {
    pub effect: Box<dyn Effect>,
    pub handle: *mut c_void,
}

impl CEffect {
    pub fn new(effect: Box<dyn Effect>, handle: *mut c_void) -> Self {
        Self {
            effect,
            handle,
        }
    }
}