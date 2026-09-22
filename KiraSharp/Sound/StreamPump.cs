using System.IO;
using System.Threading.Tasks;

namespace KiraSharp.Sound;

/// <summary>
/// Pumps a <see cref="Stream"/> into a native ring buffer on the thread pool
/// (via an async <see cref="Task"/> rather than a dedicated thread per sound).
/// The task releases its pool thread while awaiting I/O or backpressure, so an
/// idle producer doesn't tie up a thread.
/// </summary>
internal static class StreamPump {
    const int BufferBytes = 1 << 15; // 32 KiB

    /// <param name="source">Byte source to drain.</param>
    /// <param name="unitSize">Smallest indivisible write unit in bytes (1 for raw bytes, 8 for a stereo f32 frame).</param>
    /// <param name="writeUnits">
    /// Writes whole units from <c>buffer[offset..offset+byteCount]</c> (byteCount is a multiple of
    /// <paramref name="unitSize"/>) into the ring and returns the number of bytes written
    /// (a multiple of unitSize), or 0 if the ring is currently full.
    /// </param>
    /// <param name="finish">Called once the source is exhausted (mark EOF and free native resources).</param>
    public static void Start(Stream source, int unitSize, Func<byte[], int, int, int> writeUnits, Action finish) {
        _ = RunAsync(source, unitSize, writeUnits, finish);
    }

    static async Task RunAsync(Stream source, int unitSize, Func<byte[], int, int, int> writeUnits, Action finish) {
        try {
            var buffer = new byte[BufferBytes];
            int leftover = 0;
            int read;
            while ((read = await source.ReadAsync(buffer.AsMemory(leftover, buffer.Length - leftover)).ConfigureAwait(false)) > 0) {
                int total = leftover + read;
                int wholeBytes = total - (total % unitSize);
                int offset = 0;
                while (offset < wholeBytes) {
                    int wrote = writeUnits(buffer, offset, wholeBytes - offset);
                    if (wrote == 0) {
                        await Task.Delay(1).ConfigureAwait(false); // ring full; let the decoder drain
                        continue;
                    }
                    offset += wrote;
                }
                // Carry any sub-unit remainder to the next read.
                leftover = total - offset;
                if (leftover > 0) {
                    Array.Copy(buffer, offset, buffer, 0, leftover);
                }
            }
        } finally {
            source.Dispose();
            finish();
        }
    }
}
