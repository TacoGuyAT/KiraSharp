namespace KiraSharp.Effects;
public abstract class Effect {
    public unsafe void* Handle { get; protected internal set; }
    public bool IsBuilt { get; internal set; }
}
