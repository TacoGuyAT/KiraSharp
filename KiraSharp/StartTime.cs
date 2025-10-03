using KiraSharp.Generated;

namespace KiraSharp;
public struct StartTime {
    internal unsafe void* CreateHandle() => this.ClockTime is ClockTime c ? FFI.start_time(DurationMs) /* TODO: clock time */ : FFI.start_time(DurationMs);

    public double DurationMs = 0;
    public ClockTime? ClockTime = null;

    public StartTime() { }
}
