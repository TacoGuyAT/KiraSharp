using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using KiraSharp.Generated;

namespace KiraSharp.Effects;

/// <summary>
/// Base class for an effect whose DSP runs in managed C# code. Override
/// <see cref="Process"/> to transform each stereo sample frame.
///
/// WARNING: <see cref="Process"/> is invoked on the audio render thread, once
/// per sample frame. Keep it allocation-free and fast. Because it crosses into
/// managed code, a GC pause can theoretically cause an audio glitch — this is
/// fine for tools/examples but production DSP should be implemented in Rust.
///
/// The owning effect instance must be kept alive (not garbage collected) for as
/// long as the track it is attached to is playing.
/// </summary>
public abstract class CustomEffect : Effect {
    GCHandle gcHandle;

    /// <summary>
    /// Transforms one stereo sample frame in place. Runs on the audio thread.
    /// </summary>
    protected abstract void Process(ref float left, ref float right);

    /// <summary>
    /// Called once (on the thread that builds the track, before any
    /// <see cref="Process"/> call) with the renderer's sample rate. Override to
    /// set up rate-dependent state.
    /// </summary>
    protected virtual void OnSampleRate(int sampleRate) { }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
    static unsafe void Trampoline(void* userData, float* frame) {
        var self = (CustomEffect)GCHandle.FromIntPtr((nint)userData).Target!;
        self.Process(ref frame[0], ref frame[1]);
    }

    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
    static unsafe void InitTrampoline(void* userData, uint sampleRate) {
        var self = (CustomEffect)GCHandle.FromIntPtr((nint)userData).Target!;
        self.OnSampleRate((int)sampleRate);
    }

    public unsafe CustomEffect() {
        // Keep this object alive and reachable from native code via the handle.
        gcHandle = GCHandle.Alloc(this);
        delegate* unmanaged[Cdecl]<void*, float*, void> process = &Trampoline;
        delegate* unmanaged[Cdecl]<void*, uint, void> init = &InitTrampoline;
        Handle = FFI.create_managed_effect(process, init, (void*)GCHandle.ToIntPtr(gcHandle));
    }

    protected override unsafe void ReleaseHandle() {
        // A C# effect has no kira-side handle. If it was never attached to a
        // track, free the CEffect we built; once attached, the CsEffect is owned
        // by the track and dropped when the AudioManager is destroyed. The audio
        // thread may invoke the trampoline until then, so the GCHandle should only
        // be freed once the engine/track is gone (dispose the AudioManager first).
        if(!IsBuilt) {
            FFI.destroy_ceffect(Handle);
        }
        if(gcHandle.IsAllocated) {
            gcHandle.Free();
        }
    }
}
