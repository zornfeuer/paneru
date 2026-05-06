#![allow(clippy::cast_possible_truncation)]

use crate::manager::{Origin, Size};

#[must_use]
pub(crate) fn lerp_i32(from: i32, to: i32, progress: f64) -> i32 {
    let progress = progress.clamp(0.0, 1.0);
    (f64::from(from) + (f64::from(to) - f64::from(from)) * progress).round() as i32
}

#[must_use]
pub(crate) fn lerp_origin(from: Origin, to: Origin, progress: f64) -> Origin {
    Origin::new(
        lerp_i32(from.x, to.x, progress),
        lerp_i32(from.y, to.y, progress),
    )
}

#[must_use]
pub(crate) fn lerp_size(from: Size, to: Size, progress: f64) -> Size {
    let progress = progress.clamp(0.0, 1.0);
    let x = (f64::from(from.x) + (f64::from(to.x) - f64::from(from.x)) * progress)
        .round()
        .max(1.0) as i32;
    let y = (f64::from(from.y) + (f64::from(to.y) - f64::from(from.y)) * progress)
        .round()
        .max(1.0) as i32;
    Size::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_i32_endpoints() {
        assert_eq!(lerp_i32(0, 100, 0.0), 0);
        assert_eq!(lerp_i32(0, 100, 1.0), 100);
    }

    #[test]
    fn lerp_size_minimum_one() {
        let s = lerp_size(Size::new(100, 100), Size::new(10, 10), 1.0);
        assert!(s.x >= 1);
        assert!(s.y >= 1);
    }
}
