using KiraSharp.Generated;

namespace KiraSharp.Sound;

/// <summary>
/// A playing streaming sound (PCM or encoded). Wraps a kira
/// StreamingSoundHandle.
/// </summary>
public class StreamingSound : NativeResource, ISound {
    internal unsafe void* Handle;
    internal unsafe StreamingSound(void* handle) {
        this.Handle = handle;
    }
    protected override unsafe void ReleaseHandle() {
        FFI.destroy_streaming_sound_handle(Handle);
    }

    public unsafe void Pause(Tween tween) => FFI.streaming_sound_handle_pause(this.Handle, tween.CreateHandle());
    public unsafe void Resume(Tween tween) => FFI.streaming_sound_handle_resume(this.Handle, tween.CreateHandle());
    public unsafe void Stop(Tween tween) => FFI.streaming_sound_handle_stop(this.Handle, tween.CreateHandle());
    public unsafe void SetVolumeAmp(float volume, Tween? tween = null) => FFI.streaming_sound_handle_set_volume(this.Handle, FFI.volume_amp(volume), tween.CreateHandle());
    public unsafe void SetVolumeDB(float volume, Tween? tween = null) => FFI.streaming_sound_handle_set_volume(this.Handle, FFI.volume_db(volume), tween.CreateHandle());
    public unsafe void SetPanning(double panning, Tween? tween = null) => FFI.streaming_sound_handle_set_panning(this.Handle, panning, tween.CreateHandle());

    /// <summary>Loops the whole sound. Only supported for the encoded streaming path (seekable).</summary>
    public unsafe void SetLoop() => FFI.streaming_sound_handle_set_loop(this.Handle);
    /// <summary>Loops the region between <paramref name="startSeconds"/> and <paramref name="endSeconds"/>.</summary>
    public unsafe void SetLoop(double startSeconds, double endSeconds) => FFI.streaming_sound_handle_set_loop_bounded(this.Handle, startSeconds, endSeconds);
}
