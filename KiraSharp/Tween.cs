using KiraSharp.Generated;

namespace KiraSharp;
public struct Tween {
    public static Tween INSTANT = new Tween() { DurationMs = 1 };
    internal unsafe void* CreateHandle() => FFI.create_tween(
        StartTime is StartTime s ? s.CreateHandle() : FFI.start_time_immediate(),
        FFI.duration(DurationMs),
        Easing is Easing e ? e.CreateHandle() : FFI.easing_linear()
    );

    public StartTime? StartTime = null;
    public double DurationMs = 10;
    public Easing? Easing = null;

    public Tween() { }
}
