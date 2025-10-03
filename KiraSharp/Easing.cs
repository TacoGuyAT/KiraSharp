using KiraSharp.Generated;

namespace KiraSharp;
public struct Easing {
    internal unsafe void* CreateHandle() => isInt ? FFI.easing(EaseIn, EaseOut, valueInt) : FFI.easing_f(EaseIn, EaseOut, valueDouble);

    public bool EaseIn = false;
    public bool EaseOut = false;
    bool isInt = false;
    int valueInt = 0;
    double valueDouble = 0;

    public unsafe Easing(int value) {
        this.isInt = true;
        this.valueInt = value;
    }
    public unsafe Easing(double value) {
        this.valueDouble = value;
    }

    public void SetValue(int value) {
        this.isInt = true;
        valueInt = value;
    }
    public void SetValue(double value) {
        this.isInt = false;
        valueDouble = value;
    }
}
