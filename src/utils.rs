use glam::Vec3;
use num_traits::float::Float;
use std::ops::Div;

pub const GAMMA: f64 = 2.2;

/// Applies gamma correction to a color (linear -> sRGB)
pub fn linear_to_srgb<T: Float + Div<T>>(color: [T; 3], gamma: T) -> [T; 3] {
    #[cfg(target_os = "macos")]
    {
        let inv_gamma = T::from(1.0).unwrap() / gamma;
        [
            color[0].powf(inv_gamma),
            color[1].powf(inv_gamma),
            color[2].powf(inv_gamma),
        ]
    }

    #[cfg(not(target_os = "macos"))]
    {
        color
    }
}

/// Applies inverse gamma correction to a color (sRGB -> linear)
pub fn srgb_to_linear<T: Float>(color: [T; 3], gamma: T) -> [T; 3] {
    #[cfg(target_os = "macos")]
    {
        [
            color[0].powf(gamma),
            color[1].powf(gamma),
            color[2].powf(gamma),
        ]
    }

    #[cfg(not(target_os = "macos"))]
    {
        color
    }
}

pub struct ColorPalette<T: Float, const N: usize> {
    pub palette: [[T; 3]; N],
}

const CP0: ColorPalette<f64, 4> = ColorPalette {
    palette: [
        [0.263, 0.208, 0.655],
        [1.0, 0.498, 0.243],
        [0.502, 0.769, 0.914],
        [1.0, 0.965, 0.914],
    ],
};

const CP1: ColorPalette<f64, 4> = ColorPalette {
    palette: [
        [0.9568627450980393, 0.9647058823529412, 1.0],
        [0.9529411764705882, 0.7764705882352941, 0.13725490196078433],
        [0.9215686274509803, 0.5137254901960784, 0.09019607843137255],
        [0.06274509803921569, 0.21568627450980393, 0.3607843137254902],
    ],
};
