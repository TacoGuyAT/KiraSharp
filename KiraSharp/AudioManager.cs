using KiraSharp.Generated;
using KiraSharp.Sound;

namespace KiraSharp;

public class AudioManager : NativeResource {
    internal readonly unsafe void* Handle;
    internal readonly Dictionary<string, Track> tracks = [];

    public unsafe AudioManager() {
        Handle = FFI.create_audio_manager();
    }

    // Tears down the whole kira engine (renderer + audio thread). Disposing the
    // AudioManager first is the safe way to shut down: it stops the audio thread,
    // after which any remaining track/sound/effect handles can be disposed in any
    // order without racing the renderer.
    protected override unsafe void ReleaseHandle() {
        FFI.destroy_audio_manager(Handle);
    }

    public ISound Play(ISoundData soundData) {
        return soundData.Play(this);
    }

    public ISound Play(ISoundData soundData, Track track) {
        return soundData.Play(this, track);
    }

    public unsafe void SetVolumeAmp(double volume, Tween? tween = null) => FFI.audio_manager_set_volume(Handle, FFI.volume_amp(volume), tween.CreateHandle());

    public unsafe void SetVolumeDB(double volume, Tween? tween = null) => FFI.audio_manager_set_volume(Handle, FFI.volume_db(volume), tween.CreateHandle());

    public unsafe Track AddSubTrack(string key, Track track) {
        if(!track.IsBuilt) {
            track.Handle = FFI.audio_manager_add_sub_track(Handle, track.Handle);
            track.IsBuilt = true;
            tracks.Add(key, track);
            return track;
        } else {
            throw new Exception("Track was already added to audio manager");
        }
    }

    public void TryGetSubTrack(string key, out Track? track) {
        tracks.TryGetValue(key, out track);
    }
}
