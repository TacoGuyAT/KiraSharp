use std::ffi::c_void;

pub struct CSStream {

}

impl CSStream {
    unsafe fn from_raw(ptr: *mut c_void) -> Self {
        // let c = Cursor::new(AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap());
        *Box::from_raw(ptr as *mut CSStream)
    }
}