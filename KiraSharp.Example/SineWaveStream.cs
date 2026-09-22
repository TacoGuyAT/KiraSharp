using System.IO;

namespace KiraSharp.Example;

/// <summary>
/// An endless read-only <see cref="Stream"/> that synthesizes a sine tone as raw
/// PCM: interleaved stereo 32-bit little-endian floats (8 bytes per frame). The
/// tone is auto-panned left↔right by a slow LFO (baked into the per-channel
/// amplitudes) to exercise stereo. It never reports EOF — pair it with
/// <c>PcmSoundData.Endless</c>.
/// </summary>
sealed class SineWaveStream : Stream {
    const int SampleBytes = sizeof(float);
    const int FrameBytes = SampleBytes * 2;
    const double TwoPi = 2.0 * Math.PI;

    readonly double step;     // tone phase increment per frame
    readonly double panStep;  // pan LFO increment per frame
    readonly float amplitude;
    double phase;
    double panPhase;

    // Staging for the current frame's bytes, so arbitrary (non-frame-aligned)
    // read sizes work correctly.
    readonly byte[] frameBytes = new byte[FrameBytes];
    int frameBytePos = FrameBytes; // == FrameBytes forces (re)generation next read

    public SineWaveStream(double frequencyHz, uint sampleRate, float amplitude, double panHz = 0.25) {
        step = TwoPi * frequencyHz / sampleRate;
        panStep = TwoPi * panHz / sampleRate;
        this.amplitude = amplitude;
    }

    public override int Read(byte[] buffer, int offset, int count) {
        int produced = 0;
        while (produced < count) { // endless: always fills the request
            if (frameBytePos == FrameBytes) {
                float v = amplitude * (float)Math.Sin(phase);
                // Constant-power pan: angle sweeps 0..π/2 as pan goes left→right.
                double pan = 0.5 * (1.0 + Math.Sin(panPhase));
                double angle = pan * (Math.PI / 2.0);
                float left = v * (float)Math.Cos(angle);
                float right = v * (float)Math.Sin(angle);
                BitConverter.TryWriteBytes(frameBytes.AsSpan(0, SampleBytes), left);
                BitConverter.TryWriteBytes(frameBytes.AsSpan(SampleBytes, SampleBytes), right);
                frameBytePos = 0;

                phase += step;
                if (phase >= TwoPi) phase -= TwoPi;
                panPhase += panStep;
                if (panPhase >= TwoPi) panPhase -= TwoPi;
            }
            int n = Math.Min(count - produced, FrameBytes - frameBytePos);
            Array.Copy(frameBytes, frameBytePos, buffer, offset + produced, n);
            frameBytePos += n;
            produced += n;
        }
        return produced;
    }

    public override bool CanRead => true;
    public override bool CanSeek => false;
    public override bool CanWrite => false;
    public override long Length => throw new NotSupportedException();
    public override long Position {
        get => throw new NotSupportedException();
        set => throw new NotSupportedException();
    }
    public override void Flush() { }
    public override long Seek(long offset, SeekOrigin origin) => throw new NotSupportedException();
    public override void SetLength(long value) => throw new NotSupportedException();
    public override void Write(byte[] buffer, int offset, int count) => throw new NotSupportedException();
}
