using KiraSharp.Effects;
using KiraSharp.Generated;

namespace KiraSharp;
public class Track {
    public unsafe void* Handle { get; protected internal set; }
    public bool IsBuilt { get; internal set; }
    public readonly IEnumerable<Effect>? Effects;
    public unsafe Track(double volume, IEnumerable<TrackRoute>? routes, IEnumerable<Effect>? effects) {
        this.Handle = FFI.create_track_builder(volume);
        if(routes is not null) {
            foreach(var route in routes) {
                // TODO: add routes
            }
        }
        if(effects is not null) {
            foreach(var effect in effects) {
                AddEffect(effect);
            }
            Effects = effects;
        }
    }
    public unsafe Track(IEnumerable<TrackRoute>? routes, IEnumerable<Effect>? effects) {
        this.Handle = FFI.create_track_builder_default();
        if(routes is not null) {
            foreach(var route in routes) {
                // TODO: add routes
            }
        }
        if(effects is not null) {
            foreach(var effect in effects) {
                AddEffect(effect);
            }
            Effects = effects;
        }
    }
    unsafe ~Track() {
        if(!IsBuilt) {
            FFI.destroy_track_builder(Handle);
        } else {
            // TODO: destroy track handle
        }
    }

    public unsafe void SetVolumeAmp(double volume, Tween? tween = null) {
        if(tween is Tween t) {
            FFI.track_set_volume(this.Handle, FFI.volume_amp(volume), t.CreateHandle());
        } else {
            FFI.track_set_volume(this.Handle, FFI.volume_amp(volume), FFI.create_tween_default());
        }
    }
    public unsafe void SetVolumeDB(double volume) {
        FFI.track_set_volume(this.Handle, FFI.volume_db(volume), FFI.create_tween_default());
    }
    public unsafe void SetVolumeDB(double volume, Tween tween) {
        FFI.track_set_volume(this.Handle, FFI.volume_db(volume), tween.CreateHandle());
    }
    public unsafe void AddRoute(TrackRoute route) {
        // TODO: track routes
    }
    public unsafe void AddEffect(Effect effect) {
        if(IsBuilt) {
            // TODO: adding effects in runtime
            throw new Exception("Adding effects to track in runtime is unsupported. See https://github.com/tesselode/kira/issues/99");
        } else if(effect.IsBuilt) {
            // TODO: adding built effects
            throw new Exception("Adding built effects to track is unsupported. See https://github.com/tesselode/kira/issues/99");
        }
        effect.Handle = FFI.track_builder_add_effect(Handle, effect.Handle);
    }
    public void GetEffect<T>(int index, out T? effect) where T : Effect {
        if(Effects is not null && Effects.Count() > index && Effects.ElementAt(index) is T result) {
            effect = result;
        } else {
            effect = null;
        }
    }
}
