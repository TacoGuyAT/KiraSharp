using System.Runtime.CompilerServices;

namespace KiraSharp;
public static class Extensions {
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public unsafe static void* CreateHandle(this Tween? tween) {
        return (tween ?? Tween.INSTANT).CreateHandle();
    }
}
