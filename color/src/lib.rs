pub mod rgb;
pub mod hsl;

#[cfg(test)]
mod tests {
    use super::{rgb::*, hsl::{*, linear::*}};
    use num::{point::{bounds::*, _3::*}, constant::{zero::*, one::*}};

    #[test]
    fn hsl_and_linear_hsl_identity() {
        for axis in _3::all_axis(Bounds { default: 0.0, other: 1.0 }) {
            let hsl: HSL<f32> = RGB(axis).into();
            let h_sl: LinearHSL<f32> = hsl.into();
            assert_eq!(hsl, h_sl.into());
        }
        for axis in _3::all_axis(Bounds { default: 1.0, other: 0.0 }) {
            let hsl: HSL<f32> = RGB(axis).into();
            let h_sl: LinearHSL<f32> = hsl.into();
            assert_eq!(hsl, h_sl.into());
        }
    }

    #[test]
    fn rgb_and_hsl_identity() {
        let rgb = RGB::<f32>(_3::zero());
        let hsl: HSL<f32> = rgb.clone().into();
        assert_eq!(rgb, hsl.into());

        let rgb = RGB::<f32>(_3::one());
        let hsl: HSL<f32> = rgb.clone().into();
        assert_eq!(rgb, hsl.into());

        for axis in _3::all_axis(Bounds { default: 0.0, other: 1.0 }) {
            let rgb = RGB::<f32>(axis);
            let hsl: HSL<f32> = rgb.clone().into();
            assert_eq!(rgb, hsl.into());
        }
        for axis in _3::all_axis(Bounds { default: 1.0, other: 0.0 }) {
            let rgb = RGB::<f32>(axis);
            let hsl: HSL<f32> = rgb.clone().into();
            assert_eq!(rgb, hsl.into());
        }
    }
}