namespace KiraSharp.Sound;
public interface ISound {
    public void Resume(Tween tween);
    public void Pause(Tween tween);
    public void Stop(Tween tween);
    public void SetVolumeAmp(float volume, Tween? tween = null);
    public void SetVolumeDB(float volume, Tween? tween = null);
}
