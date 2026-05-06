//! Duration- and curve-based window move/resize animation configuration.

use serde::Deserialize;

/// Easing curve for window move/resize animations.
///
/// Custom cubic-bezier points may be added later without changing this enum's variants.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AnimationCurve {
    Linear,
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutQuart,
}

impl AnimationCurve {
    #[must_use]
    pub fn sample(self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t.powi(3)
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Self::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
        }
    }
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct AnimationOptions {
    pub duration_ms: Option<u64>,
    pub curve: Option<AnimationCurve>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_linear_endpoints() {
        let c = AnimationCurve::Linear;
        assert!((c.sample(0.0) - 0.0).abs() < f64::EPSILON);
        assert!((c.sample(1.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sample_clamps_oob() {
        let c = AnimationCurve::EaseOutCubic;
        assert!((c.sample(-1.0) - 0.0).abs() < f64::EPSILON);
        assert!((c.sample(2.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn ease_out_cubic_mid_gt_half() {
        let c = AnimationCurve::EaseOutCubic;
        assert!(c.sample(0.5) > 0.5);
    }

    #[test]
    fn ease_in_out_cubic_mid_near_half() {
        let c = AnimationCurve::EaseInOutCubic;
        assert!((c.sample(0.5) - 0.5).abs() < 0.01);
    }
}
