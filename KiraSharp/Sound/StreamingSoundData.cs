using System.IO;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using KiraSharp.Generated;

namespace KiraSharp.Sound;

/// <summary>
/// Streaming sound data from an *encoded* source (a seekable <see cref="Stream"/>
/// of mp3/ogg/flac/wav bytes). kira reads and seeks the stream on demand and
/// decodes lazily on its own thread — nothing is buffered or retained on the Rust
/// side. The stream must be seekable (kira seeks it to determine duration and to
/// loop).
/// </summary>
public class StreamingSoundData : ISoundData {
    readonly Func<Stream> streamFactory;

    /// <summary>If set, the sound loops (the stream is seeked back, so this works).</summary>
    public LoopRange? Loop;

    /// <param name="streamFactory">Opens the encoded byte stream. Called once per Play; must be seekable.</param>
    public StreamingSoundData(Func<Stream> streamFactory) {
        this.streamFactory = streamFactory;
    }

    /// <summary>Streams an encoded audio file from disk.</summary>
    public StreamingSoundData(string path) : this(() => File.OpenRead(path)) { }

    public unsafe ISound Play(AudioManager audioManager) {
        var (userData, len) = OpenSource();
        return Wrap(FFI.streaming_stream_play(
            audioManager.Handle, (void*)userData, &ReadTrampoline, &SeekTrampoline, &DestroyTrampoline, len));
    }

    public unsafe ISound Play(AudioManager audioManager, Track track) {
        var (userData, len) = OpenSource();
        return Wrap(FFI.streaming_stream_play_on_track(
            audioManager.Handle, (void*)userData, &ReadTrampoline, &SeekTrampoline, &DestroyTrampoline, len, track.Handle));
    }

    // Opens the stream and pins a holder for it; the native source frees it via
    // DestroyTrampoline when the sound ends (on success or decode failure).
    (nint userData, ulong length) OpenSource() {
        var stream = streamFactory();
        if (!stream.CanSeek) {
            stream.Dispose();
            throw new NotSupportedException("StreamingSoundData requires a seekable stream.");
        }
        var handle = GCHandle.Alloc(stream);
        return (GCHandle.ToIntPtr(handle), (ulong)stream.Length);
    }

    unsafe ISound Wrap(void* handle) {
        // On null the native side already dropped the source (DestroyTrampoline
        // disposed the stream and freed the GCHandle), so just report the failure.
        if (handle == null) {
            throw new Exception("Failed to start streaming sound (could not decode the audio format).");
        }
        var sound = new StreamingSound(handle);
        if (Loop is LoopRange l) {
            if (l.Equals(LoopRange.FULL)) {
                sound.SetLoop();
            } else {
                sound.SetLoop(l.Start!.Value, l.End!.Value);
            }
        }
        return sound;
    }

    static unsafe Stream Target(void* userData) => (Stream)GCHandle.FromIntPtr((nint)userData).Target!;

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
    static unsafe nuint ReadTrampoline(void* userData, byte* buf, nuint len) {
        try {
            return (nuint)Target(userData).Read(new Span<byte>(buf, (int)len));
        } catch {
            return 0; // treat as EOF
        }
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
    static unsafe long SeekTrampoline(void* userData, long offset, int origin) {
        try {
            return Target(userData).Seek(offset, (SeekOrigin)origin);
        } catch {
            return -1;
        }
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
    static unsafe void DestroyTrampoline(void* userData) {
        var gcHandle = GCHandle.FromIntPtr((nint)userData);
        (gcHandle.Target as Stream)?.Dispose();
        gcHandle.Free();
    }
}
