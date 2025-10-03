namespace KiraSharp;
public static class Extensions {
    public unsafe static void* CreateHandle(this Tween? tween) {
        return (tween ?? Tween.INSTANT).CreateHandle();
    }
}
