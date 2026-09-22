namespace KiraSharp.Effects;

/// <summary>
/// A simple custom effect implemented entirely in C#: multiplies every sample
/// by <see cref="Gain"/>. Demonstrates the <see cref="CustomEffect"/> API.
/// </summary>
public class GainEffect : CustomEffect {
    /// <summary>Linear gain multiplier. 1.0 = unchanged, 0.0 = silence.</summary>
    public volatile float Gain = 1f;

    public GainEffect() { }
    public GainEffect(float gain) {
        Gain = gain;
    }

    protected override void Process(ref float left, ref float right) {
        var g = Gain;
        left *= g;
        right *= g;
    }
}
