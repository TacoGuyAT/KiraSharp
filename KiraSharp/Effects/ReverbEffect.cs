using KiraSharp.Generated;

namespace KiraSharp.Effects;
public class ReverbEffect : Effect {
    public unsafe ReverbEffect(double feedback, double damping, double stereoWidth, double mix) {
        var builder = FFI.create_reverb_builder(feedback, damping, stereoWidth, mix);
        Handle = FFI.reverb_build(builder);
    }
    public unsafe ReverbEffect(double? feedback = null, double? damping = null, double? stereoWidth = null, double? mix = null) {
        var builder = FFI.create_reverb_builder_default();
        if(feedback is double feedbackC) {
            FFI.reverb_builder_feedback(builder, feedbackC);
        }
        if(damping is double dampingC) {
            FFI.reverb_builder_damping(builder, dampingC);
        }
        if(stereoWidth is double stereoWidthC) {
            FFI.reverb_builder_stereo_width(builder, stereoWidthC);
        }
        if(mix is double mixC) {
            FFI.reverb_builder_mix(builder, mixC);
        }
        Handle = FFI.reverb_build(builder);
    }
    unsafe ~ReverbEffect() {
        if(IsBuilt) {
             FFI.destroy_reverb_handle(Handle);
        } else {
            FFI.destroy_reverb_builder(Handle);
        }
    }
    public unsafe void SetFeedback(double feedback, Tween tween) => FFI.reverb_feedback(Handle, feedback, tween.CreateHandle());
    public unsafe void SetDamping(double damping, Tween tween) => FFI.reverb_damping(Handle, damping, tween.CreateHandle());
    public unsafe void SetStereoWidth(double stereoWidth, Tween tween) => FFI.reverb_stereo_width(Handle, stereoWidth, tween.CreateHandle());
    public unsafe void SetMix(double mix, Tween tween) => FFI.reverb_mix(Handle, mix, tween.CreateHandle());
}
