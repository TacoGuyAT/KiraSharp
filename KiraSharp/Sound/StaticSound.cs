using KiraSharp.Generated;

namespace KiraSharp.Sound;
public class StaticSound : ISound {
    internal unsafe void* Handle;
    internal unsafe StaticSound(void* handle) {
        this.Handle = handle;
    }
    unsafe ~StaticSound() {
        FFI.destroy_static_sound_handle(Handle);
    }

    public unsafe void Pause(Tween tween) => FFI.static_sound_handle_pause(this.Handle, tween.CreateHandle());
    public unsafe void Resume(Tween tween) => FFI.static_sound_handle_resume(this.Handle, tween.CreateHandle());
    public unsafe void SetVolumeAmp(float volume, Tween? tween = null) => FFI.static_sound_handle_set_volume(this.Handle, FFI.volume_amp(volume), tween.CreateHandle());
    public unsafe void SetVolumeDB(float volume, Tween? tween = null) => FFI.static_sound_handle_set_volume(this.Handle, FFI.volume_db(volume), tween.CreateHandle());
    public unsafe void Stop(Tween tween) => FFI.static_sound_handle_stop(this.Handle, tween.CreateHandle());
}
