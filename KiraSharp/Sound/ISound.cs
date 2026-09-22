namespace KiraSharp.Sound;
public interface ISound : IDisposable {
    public void Resume(Tween tween);
    public void Pause(Tween tween);
    public void Stop(Tween tween);
    public void SetVolumeAmp(float volume, Tween? tween = null);
    public void SetVolumeDB(float volume, Tween? tween = null);
    /// <summary>Panning: 0 = hard left, 0.5 = center, 1 = hard right.</summary>
    public void SetPanning(double panning, Tween? tween = null);
}
