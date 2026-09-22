//! Streaming PCM: C# generates interleaved-stereo f32 samples ahead of time and
//! pushes them, in large chunks, into a lock-free SPSC ring buffer. kira drains
//! the consumer side on its own decode thread via a minimal passthrough
//! `Decoder`. This is "streaming" (the producer fills ahead while playback
//! drains) but not real-time (the producer is not tied to the audio clock).

use std::ffi::c_void;
use std::mem;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use kira::manager::AudioManager;
use kira::sound::streaming::StreamingSoundData;
use kira::sound::streaming::Decoder;
use kira::sound::FromFileError;
use kira::track::TrackHandle;
use kira::Frame;
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

/// Largest number of frames returned from a single `decode()` call.
const MAX_DECODE_FRAMES: usize = 8192;
/// How long the decode thread sleeps when the ring is momentarily empty.
const WAIT: Duration = Duration::from_millis(1);

/// A `Decoder` that pops interleaved-stereo f32 samples from the ring buffer.
/// Its `Error` is `FromFileError` so it produces the same
/// `StreamingSoundData`/`StreamingSoundHandle` type as the encoded path,
/// letting both share one set of handle functions.
struct PcmDecoder {
    consumer: HeapConsumer<f32>,
    sample_rate: u32,
    num_frames: usize,
    finished: Arc<AtomicBool>,
}

impl Decoder for PcmDecoder {
    type Error = FromFileError;

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn num_frames(&self) -> usize {
        self.num_frames
    }

    fn decode(&mut self) -> Result<Vec<Frame>, Self::Error> {
        loop {
            // `len()` counts f32 samples; two per frame.
            let available_frames = self.consumer.len() / 2;
            if available_frames > 0 {
                let take = available_frames.min(MAX_DECODE_FRAMES);
                let mut buf = vec![0f32; take * 2];
                let popped = self.consumer.pop_slice(&mut buf);
                let frames = buf[..popped]
                    .chunks_exact(2)
                    .map(|c| Frame { left: c[0], right: c[1] })
                    .collect();
                return Ok(frames);
            }
            // Nothing buffered. If the producer is done, signal end-of-stream.
            // (Safe only because the producer promises to push exactly
            // `num_frames` frames, so once empty+finished, playback has already
            // reached the end and `decode` is no longer called.)
            if self.finished.load(Ordering::Acquire) {
                return Ok(Vec::new());
            }
            thread::sleep(WAIT);
        }
    }

    fn seek(&mut self, index: usize) -> Result<usize, Self::Error> {
        // A live PCM stream can't seek; report that we landed where asked.
        // Forward-only playback never actually calls this.
        Ok(index)
    }
}

/// The C#-side handle: owns the producer (for writing) and the built sound data
/// (until it's consumed by `play`).
pub struct PcmStream {
    producer: HeapProducer<f32>,
    data: Option<StreamingSoundData<FromFileError>>,
    finished: Arc<AtomicBool>,
}

/// Creates a streaming PCM source. `total_frames` is the exact number of stereo
/// frames the caller will push (kira needs the length up front). `capacity_frames`
/// bounds the ring buffer; the producer should keep it filled ahead of playback.
#[no_mangle]
pub unsafe extern "C" fn pcm_stream_create(
    sample_rate: u32,
    total_frames: usize,
    capacity_frames: usize,
) -> *mut c_void {
    let rb = HeapRb::<f32>::new(capacity_frames * 2);
    let (producer, consumer) = rb.split();
    let finished = Arc::new(AtomicBool::new(false));
    let decoder = PcmDecoder {
        consumer,
        sample_rate,
        num_frames: total_frames,
        finished: finished.clone(),
    };
    let stream = PcmStream {
        producer,
        data: Some(StreamingSoundData::from_decoder(decoder)),
        finished,
    };
    Box::into_raw(Box::new(stream)) as *mut c_void
}

/// Pushes up to `frame_count` interleaved-stereo frames (`samples` length must be
/// `frame_count * 2`). Returns the number of frames actually written; the caller
/// should retry the remainder if the ring was full.
#[no_mangle]
pub unsafe extern "C" fn pcm_stream_write(
    stream_ptr: *mut c_void,
    samples: *const f32,
    frame_count: usize,
) -> usize {
    let stream = &mut *(stream_ptr as *mut PcmStream);
    let slice = std::slice::from_raw_parts(samples, frame_count * 2);
    // push_slice may write an odd count only if free space is odd; we keep
    // capacity and every push frame-aligned, so the occupied count stays even.
    stream.producer.push_slice(slice) / 2
}

/// Signals that no more frames will be pushed.
#[no_mangle]
pub unsafe extern "C" fn pcm_stream_finish(stream_ptr: *mut c_void) {
    let stream = &mut *(stream_ptr as *mut PcmStream);
    stream.finished.store(true, Ordering::Release);
}

/// Plays the stream on the main track. Returns a `StreamingSoundHandle`.
#[no_mangle]
pub unsafe extern "C" fn pcm_stream_play(
    audio_manager_ptr: *mut c_void,
    stream_ptr: *mut c_void,
) -> *mut c_void {
    play(audio_manager_ptr, stream_ptr, None)
}

/// Plays the stream routed to the given sub-track. Returns a `StreamingSoundHandle`.
#[no_mangle]
pub unsafe extern "C" fn pcm_stream_play_on_track(
    audio_manager_ptr: *mut c_void,
    stream_ptr: *mut c_void,
    track_handle_ptr: *mut c_void,
) -> *mut c_void {
    play(audio_manager_ptr, stream_ptr, Some(track_handle_ptr))
}

unsafe fn play(
    audio_manager_ptr: *mut c_void,
    stream_ptr: *mut c_void,
    track_handle_ptr: Option<*mut c_void>,
) -> *mut c_void {
    let stream = &mut *(stream_ptr as *mut PcmStream);
    let mut data = stream.data.take().expect("PCM stream was already played");
    if let Some(track_ptr) = track_handle_ptr {
        let destination = &mut *(track_ptr as *mut TrackHandle);
        data.settings.output_destination = destination.deref().into();
    }
    let mut audio_manager = Box::from_raw(audio_manager_ptr as *mut AudioManager);
    let handle = audio_manager.play(data).expect("Failed to play PCM stream");
    mem::forget(audio_manager);
    Box::into_raw(Box::new(handle)) as *mut c_void
}

/// Frees the stream handle (and its producer). The played sound keeps the
/// consumer side alive independently.
#[no_mangle]
pub unsafe extern "C" fn destroy_pcm_stream(stream_ptr: *mut c_void) {
    let stream = Box::from_raw(stream_ptr as *mut PcmStream);
    stream.finished.store(true, Ordering::Release);
    drop(stream);
}
