using System.IO;
using KiraSharp.Generated;

namespace KiraSharp.Sound;

/// <summary>
/// Streaming PCM sound data read from a C# <see cref="Stream"/> of raw audio:
/// interleaved stereo 32-bit little-endian floats (8 bytes per frame). The
/// stream is pumped, ahead of playback and in large chunks, into a native ring
/// buffer; kira drains it on its own decode thread. Reading the stream is not
/// tied to the audio clock (non-real-time).
/// </summary>
public class PcmSoundData : ISoundData {
    /// <summary>Pass as <c>totalFrames</c> for an endless stream (plays until stopped).</summary>
    public const long Endless = -1;

    readonly Func<Stream> streamFactory;
    readonly uint sampleRate;
    readonly long totalFrames;
    readonly int capacityFrames;
    const int BytesPerFrame = sizeof(float) * 2; // 2 channels

    /// <param name="streamFactory">Opens the raw-PCM stream (stereo f32 LE). Called once per Play.</param>
    /// <param name="sampleRate">Sample rate of the PCM data, in Hz.</param>
    /// <param name="totalFrames">Total number of stereo frames the stream will yield (kira needs the length up front), or <see cref="Endless"/> for an unbounded stream.</param>
    /// <param name="capacityFrames">Ring buffer size in frames. Larger = more read-ahead.</param>
    public PcmSoundData(Func<Stream> streamFactory, uint sampleRate, long totalFrames, int capacityFrames = 1 << 15) {
        this.streamFactory = streamFactory;
        this.sampleRate = sampleRate;
        this.totalFrames = totalFrames;
        this.capacityFrames = capacityFrames;
    }

    // Endless maps to usize::MAX on the Rust side, so the transport never reaches
    // the end (~millions of years at audio rates).
    nuint FrameCount => totalFrames < 0 ? nuint.MaxValue : (nuint)totalFrames;

    public unsafe ISound Play(AudioManager audioManager) {
        var stream = FFI.pcm_stream_create(sampleRate, FrameCount, (nuint)capacityFrames);
        // Play first (the sound data is already built, no probing), then start the
        // pump — so the main thread and the pump never alias the native handle.
        var handle = FFI.pcm_stream_play(audioManager.Handle, stream);
        StartProducer(stream);
        return new StreamingSound(handle);
    }

    public unsafe ISound Play(AudioManager audioManager, Track track) {
        var stream = FFI.pcm_stream_create(sampleRate, FrameCount, (nuint)capacityFrames);
        var handle = FFI.pcm_stream_play_on_track(audioManager.Handle, stream, track.Handle);
        StartProducer(stream);
        return new StreamingSound(handle);
    }

    // Reads the PCM stream on the thread pool and pushes frames into the native
    // ring buffer ahead of the decoder. Frees the producer when done.
    unsafe void StartProducer(void* streamPtr) {
        nint ptr = (nint)streamPtr;

        int WriteUnits(byte[] b, int offset, int byteCount) {
            int frames = byteCount / BytesPerFrame;
            fixed (byte* bp = &b[offset]) {
                return (int)FFI.pcm_stream_write((void*)ptr, (float*)bp, (nuint)frames) * BytesPerFrame;
            }
        }
        void Finish() {
            FFI.pcm_stream_finish((void*)ptr);
            FFI.destroy_pcm_stream((void*)ptr);
        }

        StreamPump.Start(streamFactory(), BytesPerFrame, WriteUnits, Finish);
    }
}
