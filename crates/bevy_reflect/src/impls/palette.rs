use crate::prelude::ReflectDefault;
use crate::{self as bevy_reflect};
use bevy_reflect_derive::{impl_from_reflect_value, impl_reflect_struct, impl_reflect_value};

use palette::*;

impl_reflect_struct!(
    #[reflect(Debug, PartialEq, Default)]
    struct Srgba {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    }
);

impl_reflect_struct!(
    #[reflect(Debug, PartialEq, Default)]
    struct LinSrgba {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    }
);

impl_reflect_value!(RgbHue(Debug, PartialEq, Default));
impl_from_reflect_value!(RgbHue);
impl_reflect_struct!(
    #[reflect(Debug, PartialEq, Default)]
    struct Hsla {
        hue: RgbHue,
        saturation: f32,
        lightness: f32,
        alpha: f32,
    }
);

impl_reflect_value!(LabHue(Debug, PartialEq, Default));
impl_from_reflect_value!(LabHue);
// impl_reflect_struct!(
//     #[reflect(Debug, PartialEq, Default)]
//     struct Lcha {
//         l: f32,
//         chroma: f32,
//         hue: LabHue,
//         alpha: f32,
//     }
// );
impl_reflect_struct!(
    #[reflect(Debug, PartialEq, Default)]
    struct Lch {
        l: f32,
        chroma: f32,
        hue: LabHue,
    }
);
impl_reflect_struct!(
    // #[reflect(Default)]
    struct Alpha<Lch, f32> {
        color: Lch,
        alpha: f32,
    }
);
