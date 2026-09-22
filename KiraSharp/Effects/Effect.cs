namespace KiraSharp.Effects;
public abstract class Effect : NativeResource {
    public unsafe void* Handle { get; protected internal set; }
    public bool IsBuilt { get; internal set; }
}
