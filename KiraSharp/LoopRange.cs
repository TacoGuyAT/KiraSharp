namespace KiraSharp;

// TODO: Match with its Rust counterpart
public struct LoopRange : IEquatable<LoopRange> {
    public static LoopRange FULL = new LoopRange { Start = null, End = null };
    public double? Start = null;
    public double? End = null;
    public LoopRange(double start, double end) {
        this.Start = start;
        this.End = end;
    }

    public bool Equals(LoopRange other) {
        return Start == other.Start && End == other.End;
    }
}
