using KiraSharp.Generated;
using KiraSharp.Sound;

namespace KiraSharp;

public class AudioManager {
    internal unsafe void* Handle;
    Dictionary<string, Track> tracks = new();
    public unsafe AudioManager() {
        Handle = FFI.create_audio_manager();
    }
    unsafe ~AudioManager() {
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
