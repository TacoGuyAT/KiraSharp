namespace KiraSharp.Effects;

/// <summary>
/// A custom (C#) effect that shifts every frequency by a constant offset in Hz
/// — single-sideband modulation, not pitch shifting, so harmonic ratios are not
/// preserved (the sound becomes inharmonic/metallic for non-trivial shifts).
///
/// Implemented by forming the analytic signal with a Hilbert-transform FIR
/// (quadrature path) plus a matched delay (in-phase path), then mixing with a
/// complex oscillator: <c>y = i·cos(φ) − q·sin(φ)</c>, <c>φ += 2π·shift/fs</c>.
///
/// Runs per-sample on the audio thread (see <see cref="CustomEffect"/>); the
/// FIR cost is O(taps) per sample.
/// </summary>
public class FrequencyShiftEffect : CustomEffect {
    const double TwoPi = 2.0 * Math.PI;

    /// <summary>Shift amount in Hz (may be negative). Read once per sample.</summary>
    public volatile float ShiftHz;

    int sampleRate = 44100;

    readonly int taps;
    readonly int center;
    readonly float[] kernel;          // Hilbert FIR (anti-symmetric, even taps zero)
    readonly float[] left;            // circular delay lines
    readonly float[] right;
    int pos = -1;
    double phase;

    public FrequencyShiftEffect(float shiftHz = 0f, int taps = 101) {
        if ((taps & 1) == 0) {
            taps++; // must be odd so the in-phase delay is an integer
        }
        this.taps = taps;
        center = taps / 2;
        kernel = new float[taps];
        for (int k = 0; k < taps; k++) {
            int m = k - center;
            // Ideal Hilbert response: 0 for even offsets, 2/(π·m) for odd.
            float h = (m & 1) == 0 ? 0f : 2f / (MathF.PI * m);
            // Hamming window.
            float window = 0.54f - 0.46f * MathF.Cos(2f * MathF.PI * k / (taps - 1));
            kernel[k] = h * window;
        }
        left = new float[taps];
        right = new float[taps];
        ShiftHz = shiftHz;
    }

    protected override void OnSampleRate(int sr) => sampleRate = sr;

    protected override void Process(ref float l, ref float r) {
        if (++pos >= taps) {
            pos = 0;
        }
        left[pos] = l;
        right[pos] = r;

        // In-phase path: input delayed by the FIR's group delay (center).
        int delayed = pos - center;
        if (delayed < 0) {
            delayed += taps;
        }
        float li = left[delayed];
        float ri = right[delayed];

        // Quadrature path: convolution with the Hilbert kernel.
        float lq = 0f, rq = 0f;
        int idx = pos;
        for (int k = 0; k < taps; k++) {
            float h = kernel[k];
            if (h != 0f) {
                lq += h * left[idx];
                rq += h * right[idx];
            }
            if (--idx < 0) {
                idx += taps;
            }
        }

        // Mix with the complex oscillator to shift the spectrum.
        phase += TwoPi * ShiftHz / sampleRate;
        if (phase >= Math.PI) {
            phase -= TwoPi;
        } else if (phase < -Math.PI) {
            phase += TwoPi;
        }
        float cos = (float)Math.Cos(phase);
        float sin = (float)Math.Sin(phase);

        l = li * cos - lq * sin;
        r = ri * cos - rq * sin;
    }
}
