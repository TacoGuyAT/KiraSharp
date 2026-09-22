using KiraSharp;
using KiraSharp.Effects;
using KiraSharp.Example;
using KiraSharp.Sound;
using NativeFileDialogSharp;

Console.WriteLine("KiraSharp example. Choose a source:");
Console.WriteLine("  1) Play a file              (static sound)");
Console.WriteLine("  2) Stream a file            (streaming sound data, lazy decode)");
Console.WriteLine("  3) Play a generated tone    (streaming PCM sine wave)");
Console.Write("> ");

ISoundData? soundData = null;
// The sine tone is already panned in the generator; for the file modes we
// instead sweep panning at runtime through the tweened handle API.
switch (Console.ReadKey(intercept: true).KeyChar) {
    case '1':
        if (TryPickFile(out var path1)) {
            soundData = new StaticSoundData(path1) { Loop = LoopRange.FULL };
        }
        break;
    case '2':
        if (TryPickFile(out var path2)) {
            soundData = new StreamingSoundData(path2) { Loop = LoopRange.FULL };
        }
        break;
    case '3':
        soundData = new PcmSoundData(
            () => new SineWaveStream(frequencyHz: 440.0, sampleRate: 44100, amplitude: 0.5f),
            sampleRate: 44100,
            totalFrames: PcmSoundData.Endless);
        break;
}

Console.WriteLine();
if (soundData is null) {
    Console.WriteLine("Nothing selected.");
    return;
}

var manager = new AudioManager();

// Built-in (Rust/kira) reverb effect.
// var reverb = new ReverbEffect(feedback: 0.85, damping: 0.1, stereoWidth: 1.0, mix: 0.25);

// Custom effects whose DSP runs in C# (see GainEffect / FrequencyShiftEffect).
var gain = new GainEffect(0.8f);
var freqShift = new FrequencyShiftEffect(0f);

// A sub-track carrying both effects (applied in order: gain, then shift).
var track = manager.AddSubTrack("main", new Track(1.0, null, new Effect[] { gain, freqShift }));

ISound sound = manager.Play(soundData, track);

Console.WriteLine();
Console.WriteLine("Controls:");
Console.WriteLine("  Up / Down    : C# gain effect       +/- 0.1");
Console.WriteLine("  Left / Right : C# frequency shift   +/- 25 Hz");
Console.WriteLine("  Esc          : quit");
Console.WriteLine($"  Gain = {gain.Gain:0.0}   Shift = {freqShift.ShiftHz:0} Hz");

while (true) {
    var key = Console.ReadKey(intercept: true).Key;
    if (key == ConsoleKey.Escape) {
        break;
    } else if (key == ConsoleKey.UpArrow) {
        gain.Gain = MathF.Min(2f, gain.Gain + 0.1f);
    } else if (key == ConsoleKey.DownArrow) {
        gain.Gain = MathF.Max(0f, gain.Gain - 0.1f);
    } else if (key == ConsoleKey.RightArrow) {
        freqShift.ShiftHz += 25f;
    } else if (key == ConsoleKey.LeftArrow) {
        freqShift.ShiftHz -= 25f;
    } else {
        continue;
    }
    Console.WriteLine($"  Gain = {gain.Gain:0.0}   Shift = {freqShift.ShiftHz:0} Hz");
}

// Keep the AudioManager rooted until here: in Release builds the GC could
// otherwise finalize this no-longer-referenced local during the ReadKey loop and
// tear down the engine mid-playback. The track and effects are reachable through
// it (and the gain effect also self-roots via its GCHandle), so one is enough.
// (A long-running app would instead Dispose these when done — manager first.)
// GC.KeepAlive(manager);

static bool TryPickFile(out string path) {
    var result = Dialog.FileOpen("wav,mp3,ogg,flac");
    if (result.IsOk) {
        path = result.Path;
        Console.WriteLine($"Selected: {path}");
        return true;
    }
    Console.WriteLine(result.IsCancelled ? "Cancelled." : $"Dialog error: {result.ErrorMessage}");
    path = "";
    return false;
}
