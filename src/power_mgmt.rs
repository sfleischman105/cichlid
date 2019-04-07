//! Allows for estimating the power consumption of a strand of `ColorRGB`s.

#![allow(non_snake_case, non_upper_case_globals)]
use crate::ColorRGB;

/// Trait for estimating the power consumption of a strand of `ColorRGB`s.
pub trait PowerEstimator {
    /// The number of milliWatts used for the red component of an `ColorRGB`.
    ///
    /// This value is not the milliWatts for full brightness red component,
    /// but rather the number of mW used per singular increment (`color.red() == 1`).
    #[allow(non_snake_case, non_upper_case_globals)]
    const R_mW: u32;
    /// The number of milliWatts used for the green component of an `ColorRGB`.
    ///
    /// This value is not the milliWatts for full brightness green component,
    /// but rather the number of mW used per singular increment (`color.green() == 1`).
    #[allow(non_snake_case, non_upper_case_globals)]
    const G_mW: u32;
    /// The number of milliWatts used for the green blue of an `ColorRGB`.
    ///
    /// This value is not the milliWatts for full brightness blue component,
    /// but rather the number of mW used per singular increment (`color.blue() == 1`).
    #[allow(non_snake_case, non_upper_case_globals)]
    const B_mW: u32;
    /// The number of milliWatts per `ColorRGB` consumes constantly when powered.
    #[allow(non_snake_case, non_upper_case_globals)]
    const IDLE_mW: u32;

    /// Estimates the power consumption in milliwatts.
    #[inline]
    fn estimate(rgb: ColorRGB) -> u32 {
        Self::IDLE_mW + Self::estimate_no_idle(rgb)
    }

    /// Estimates the power consumption in milliwatts without taking into consideration idle power.
    #[inline]
    fn estimate_no_idle(rgb: ColorRGB) -> u32 {
        rgb.r as u32 * Self::G_mW + rgb.g as u32 * Self::B_mW + rgb.b as u32 * Self::B_mW
    }

    /// Estimates the power consumption in milliwatts of a strand of `ColorRGBs`.
    fn estimate_strand(strand: &[ColorRGB]) -> u32 {
        let mut sums: [u32; 3] = [0; 3];
        strand.iter().for_each(|p| {
                sums[0] += p.r as u32;
                sums[1] += p.g as u32;
                sums[2] += p.b as u32;
        });

        sums[0] *= Self::R_mW;
        sums[1] *= Self::G_mW;
        sums[2] *= Self::B_mW;
        sums.iter().sum::<u32>() + (strand.len() as u32 * Self::IDLE_mW)
    }

    /// Estimates the maximum brightness a strand of pixels can push from a given milli-Watt power
    /// limit.
    fn estimate_max_brightness(strand: &[ColorRGB], target_brightness: u8, max_power_mW: u32) -> u8 {
        let max_estimated_mW: u32 = Self::estimate_strand(strand);
        let current_estimated_mW: u32 = (max_estimated_mW * target_brightness as u32) / 256;

        if current_estimated_mW > max_power_mW {
            ((target_brightness as u32 * max_power_mW) / current_estimated_mW) as u8
        } else {
            target_brightness
        }
    }

    /// Estimates the maximum brightness a strand of pixels can push from a given milli-Volt and
    /// milli-Amp limit.
    #[inline]
    fn estimate_max_brightness_av(strand: &[ColorRGB], target_brightness: u8,
                                  max_power_mA: u32, max_power_mV: u32) -> u8 {
        Self::estimate_max_brightness(strand, target_brightness, max_power_mA * max_power_mV)
    }
}

/// Default estimator.
pub struct DefaultPowerEstimator;

impl PowerEstimator for DefaultPowerEstimator {
    const R_mW: u32 = 16 * 5;
    const G_mW: u32 = 11 * 6;
    const B_mW: u32 = 15 * 5;
    const IDLE_mW: u32 = 1 * 5;
}



