namespace KiraSharp.Sound;
public interface ISoundData {
    public ISound Play(AudioManager audioManager);
    public ISound Play(AudioManager audioManager, Track track);
}
