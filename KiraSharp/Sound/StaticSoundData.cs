using KiraSharp.Generated;
using System.Runtime.InteropServices;

namespace KiraSharp.Sound;
public struct StaticSoundData : ISoundData
{
    internal unsafe void* CreateHandle(AudioManager audioManager)
    {
        void* handle;
        if (bytes is Func<byte[]> b)
        {
            var bytesResult = b.Invoke();
            fixed (byte* bytesPointer = bytesResult)
            {
                handle = FFI.create_static_sound_data_from_bytes(bytesPointer, (nuint)bytesResult.Length);
            }
        }
        else
        {
            // Rust only borrows the string (via CStr) for the duration of the
            // call, so free the unmanaged copy as soon as it returns.
            IntPtr pathPtr = Marshal.StringToHGlobalAnsi(path);
            try
            {
                handle = FFI.create_static_sound_data_from_path((byte*)pathPtr);
            }
            finally
            {
                Marshal.FreeHGlobal(pathPtr);
            }
        }
        if (volume is double v)
        {
            FFI.static_sound_data_volume(handle, isAmp ? FFI.volume_amp(v) : FFI.volume_db(v));
        }
        if (Pan is double p)
        {
            FFI.static_sound_data_panning(handle, p);
        }
        if (Loop is LoopRange l)
        {
            if (l.Equals(LoopRange.FULL))
            {
                FFI.static_sound_data_loop(handle);
            }
            else
            {
                //FFI.static_sound_data_loop_bounded(handle, l.Start, l.End);
            }
        }
        return handle;
    }

    public double? VolumeAmp
    {
        get => volume;
        set
        {
            isAmp = true;
            volume = value;
        }
    }
    public double? VolumeDB
    {
        get => volume;
        set
        {
            isAmp = false;
            volume = value;
        }
    }
    bool isAmp;
    public double? Pan;
    public Track? Output;
    public LoopRange? Loop;
    Func<byte[]>? bytes;
    string? path;
    double? volume;

    public unsafe StaticSoundData(Func<byte[]> bytes)
    {
        this.bytes = bytes;
    }
    public unsafe StaticSoundData(string path)
    {
        this.path = path;
    }

    public unsafe ISound Play(AudioManager audioManager)
    {
        return new StaticSound(FFI.static_sound_data_play(audioManager.Handle, CreateHandle(audioManager)));
    }
    public unsafe ISound Play(AudioManager audioManager, Track track)
    {
        var handle = CreateHandle(audioManager);
        FFI.static_sound_data_output_destination_track_handle(handle, track.Handle);
        return new StaticSound(FFI.static_sound_data_play(audioManager.Handle, handle));
    }
}
