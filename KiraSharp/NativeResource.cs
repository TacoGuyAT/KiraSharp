using System.Threading;

namespace KiraSharp;

/// <summary>
/// Base class for objects that own a native (Rust) resource. Prefer deterministic
/// cleanup via <see cref="Dispose"/> (or a <c>using</c> statement); the finalizer
/// is only a safety net for forgotten disposes. Releasing happens exactly once,
/// whichever path runs first.
/// </summary>
public abstract class NativeResource : IDisposable {
    int released; // 0 = live, 1 = released (guards against double-free / races)

    /// <summary>Frees the underlying native resource. Invoked exactly once.</summary>
    protected abstract void ReleaseHandle();

    public void Dispose() {
        if (Interlocked.Exchange(ref released, 1) == 0) {
            ReleaseHandle();
        }
        GC.SuppressFinalize(this);
    }

    ~NativeResource() {
        if (Interlocked.Exchange(ref released, 1) == 0) {
            ReleaseHandle();
        }
    }
}
