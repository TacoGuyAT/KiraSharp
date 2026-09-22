//! Streaming sound data from *undecoded* (encoded) audio backed by a C#
//! `Stream`. A `MediaSource` adapter calls back into C# to read and seek the
//! stream on demand — nothing is retained on the Rust side. kira decodes lazily
//! on its own thread, and seeking (for looping or duration probing) goes
//! straight to the underlying stream, so the C# stream must be seekable.

use std::ffi::c_void;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem;
use std::ops::Deref;

use kira::manager::AudioManager;
use kira::sound::streaming::StreamingSoundData;
use kira::sound::FromFileError;
use kira::track::TrackHandle;
use symphonia::core::io::MediaSource;

/// Reads up to `len` bytes into `buf`; returns bytes read (0 = EOF).
type ReadFn = unsafe extern "C" fn(user_data: *mut c_void, buf: *mut u8, len: usize) -> usize;
/// Seeks the stream. `origin`: 0 = start, 1 = current, 2 = end. Returns the new
/// absolute position, or a negative value on error.
type SeekFn = unsafe extern "C" fn(user_data: *mut c_void, offset: i64, origin: i32) -> i64;
/// Releases the C# side (dispose the stream, free the handle).
type DestroyFn = unsafe extern "C" fn(user_data: *mut c_void);

/// A seekable `MediaSource` that delegates reads/seeks to a C# `Stream`. Holds no
/// audio data; `Drop` invokes the C# `destroy` callback to release the stream.
struct CsMediaSource {
    user_data: *mut c_void,
    read: ReadFn,
    seek: SeekFn,
    destroy: DestroyFn,
    total_len: u64,
}

// kira moves the decoder (and this source) to its decode thread; only one thread
// ever touches it at a time (probe on the calling thread, then the decode thread).
unsafe impl Send for CsMediaSource {}
unsafe impl Sync for CsMediaSource {}

impl Read for CsMediaSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(unsafe { (self.read)(self.user_data, buf.as_mut_ptr(), buf.len()) })
    }
}

impl Seek for CsMediaSource {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let (origin, offset) = match pos {
            SeekFrom::Start(offset) => (0, offset as i64),
            SeekFrom::Current(offset) => (1, offset),
            SeekFrom::End(offset) => (2, offset),
        };
        let result = unsafe { (self.seek)(self.user_data, offset, origin) };
        if result < 0 {
            Err(io::Error::new(io::ErrorKind::Other, "C# stream seek failed"))
        } else {
            Ok(result as u64)
        }
    }
}

impl MediaSource for CsMediaSource {
    fn is_seekable(&self) -> bool {
        true
    }
    fn byte_len(&self) -> Option<u64> {
        if self.total_len > 0 {
            Some(self.total_len)
        } else {
            None
        }
    }
}

impl Drop for CsMediaSource {
    fn drop(&mut self) {
        unsafe { (self.destroy)(self.user_data) };
    }
}

/// Plays a streaming sound backed by a C# stream on the main track. Returns a
/// `StreamingSoundHandle`, or null if the format could not be determined. On any
/// outcome the source is consumed and the `destroy` callback eventually runs.
#[no_mangle]
pub unsafe extern "C" fn streaming_stream_play(
    audio_manager_ptr: *mut c_void,
    user_data: *mut c_void,
    read: ReadFn,
    seek: SeekFn,
    destroy: DestroyFn,
    total_len: u64,
) -> *mut c_void {
    let source = CsMediaSource { user_data, read, seek, destroy, total_len };
    play(audio_manager_ptr, source, None)
}

/// Plays a streaming sound backed by a C# stream, routed to the given sub-track.
#[no_mangle]
pub unsafe extern "C" fn streaming_stream_play_on_track(
    audio_manager_ptr: *mut c_void,
    user_data: *mut c_void,
    read: ReadFn,
    seek: SeekFn,
    destroy: DestroyFn,
    total_len: u64,
    track_handle_ptr: *mut c_void,
) -> *mut c_void {
    let source = CsMediaSource { user_data, read, seek, destroy, total_len };
    play(audio_manager_ptr, source, Some(track_handle_ptr))
}

unsafe fn play(
    audio_manager_ptr: *mut c_void,
    source: CsMediaSource,
    track_handle_ptr: Option<*mut c_void>,
) -> *mut c_void {
    // Symphonia probes the format here, reading/seeking the C# stream. On error
    // `source` is dropped (its destroy callback releases the C# side).
    let mut data = match StreamingSoundData::from_media_source(source) {
        Ok(data) => data,
        Err(_) => return std::ptr::null_mut(),
    };
    if let Some(track_ptr) = track_handle_ptr {
        let destination = &mut *(track_ptr as *mut TrackHandle);
        data.settings.output_destination = destination.deref().into();
    }
    let mut audio_manager = Box::from_raw(audio_manager_ptr as *mut AudioManager);
    let handle = audio_manager
        .play(data)
        .expect("Failed to play streaming sound");
    mem::forget(audio_manager);
    Box::into_raw(Box::new(handle)) as *mut c_void
}

/// Shared streaming-sound-handle operations. Both the PCM and encoded paths
/// produce `StreamingSoundData<FromFileError>` and therefore the same handle
/// type, so these work for both.
mod handle {
    use super::*;
    use kira::sound::streaming::StreamingSoundHandle;
    use kira::tween::Tween;
    use kira::Volume;

    type Handle = StreamingSoundHandle<FromFileError>;

    #[no_mangle]
    pub unsafe extern "C" fn destroy_streaming_sound_handle(handle_ptr: *mut c_void) {
        drop(Box::from_raw(handle_ptr as *mut Handle));
    }

    #[no_mangle]
    pub unsafe extern "C" fn streaming_sound_handle_resume(
        handle_ptr: *mut c_void,
        tween_ptr: *mut c_void,
    ) {
        let handle = &mut *(handle_ptr as *mut Handle);
        let tween = *Box::from_raw(tween_ptr as *mut Tween);
        handle.resume(tween);
    }

    #[no_mangle]
    pub unsafe extern "C" fn streaming_sound_handle_pause(
        handle_ptr: *mut c_void,
        tween_ptr: *mut c_void,
    ) {
        let handle = &mut *(handle_ptr as *mut Handle);
        let tween = *Box::from_raw(tween_ptr as *mut Tween);
        handle.pause(tween);
    }

    #[no_mangle]
    pub unsafe extern "C" fn streaming_sound_handle_stop(
        handle_ptr: *mut c_void,
        tween_ptr: *mut c_void,
    ) {
        let handle = &mut *(handle_ptr as *mut Handle);
        let tween = *Box::from_raw(tween_ptr as *mut Tween);
        handle.stop(tween);
    }

    #[no_mangle]
    pub unsafe extern "C" fn streaming_sound_handle_set_volume(
        handle_ptr: *mut c_void,
        volume_ptr: *mut c_void,
        tween_ptr: *mut c_void,
    ) {
        let handle = &mut *(handle_ptr as *mut Handle);
        let volume = *Box::from_raw(volume_ptr as *mut Volume);
        let tween = *Box::from_raw(tween_ptr as *mut Tween);
        handle.set_volume(volume, tween);
    }

    /// Panning: 0.0 = hard left, 0.5 = center, 1.0 = hard right. Smoothly
    /// interpolated by the given tween.
    #[no_mangle]
    pub unsafe extern "C" fn streaming_sound_handle_set_panning(
        handle_ptr: *mut c_void,
        panning: f64,
        tween_ptr: *mut c_void,
    ) {
        let handle = &mut *(handle_ptr as *mut Handle);
        let tween = *Box::from_raw(tween_ptr as *mut Tween);
        handle.set_panning(panning, tween);
    }

    /// Loops the whole sound. Requires a seekable decoder (the encoded path is;
    /// the live PCM path is not).
    #[no_mangle]
    pub unsafe extern "C" fn streaming_sound_handle_set_loop(handle_ptr: *mut c_void) {
        let handle = &mut *(handle_ptr as *mut Handle);
        handle.set_loop_region(..);
    }

    /// Loops the region between `start` and `end` (in seconds).
    #[no_mangle]
    pub unsafe extern "C" fn streaming_sound_handle_set_loop_bounded(
        handle_ptr: *mut c_void,
        start: f64,
        end: f64,
    ) {
        let handle = &mut *(handle_ptr as *mut Handle);
        handle.set_loop_region(start..end);
    }
}
