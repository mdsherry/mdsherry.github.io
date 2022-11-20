#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowedTransformFamiles {
    NoTransforms,
    Rotate,
    RotateAndFlip,
}
use AllowedTransformFamiles::*;

use crate::app::bag_draw::count3;

pub trait Transform {
    const ORBITS: u64;
    const ORBIT_SIZE: u64 = 2;
    fn applicable(allowed_families: AllowedTransformFamiles, square: bool) -> bool;
    fn base_fixed(w: u64, h: u64) -> u64;
    fn free(w: u64, h: u64) -> u64 {
        let fixed = Self::base_fixed(w, h);
        let unfixed = w * h - fixed * Self::ORBIT_SIZE;
        fixed + unfixed
    }
    fn n_fixed(w: u64, h: u64, n_colours: u64) -> u64 {
        n_colours.pow(Self::free(w, h) as u32)
    }
    fn total_fixed(w: u64, h: u64, n_colours: u64) -> u64 {
        Self::n_fixed(w, h, n_colours) * Self::ORBITS
    }
    fn limited_n_fixed(w: u64, h: u64, n_colours: u64, max_repeats: u64) -> u64 {
        let mut counts = vec![0; max_repeats as usize + 1];
        counts[max_repeats as usize] = n_colours;
        let fixed = Self::base_fixed(w, h);
        let mut draws = vec![Self::ORBIT_SIZE as u8; fixed as usize];
        draws.resize(Self::free(w, h) as usize, 1);
        count3(&draws, &mut counts)
    }
    fn limited_total_fixed(w: u64, h: u64, n_colours: u64, max_repeats: u64) -> u64 {
        Self::limited_n_fixed(w, h, n_colours, max_repeats) * Self::ORBITS
    }
}

pub struct NoXform;
impl Transform for NoXform {
    const ORBITS: u64 = 1;
    const ORBIT_SIZE: u64 = 1;
    fn applicable(_allowed_families: AllowedTransformFamiles, _square: bool) -> bool {
        true
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        w * h
    }
}

pub struct Rot90;
impl Transform for Rot90 {
    const ORBITS: u64 = 2;
    const ORBIT_SIZE: u64 = 4;

    fn applicable(allowed_families: AllowedTransformFamiles, square: bool) -> bool {
        matches!(allowed_families, Rotate | RotateAndFlip) && square
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        assert_eq!(w, h);
        if w % 2 == 1 {
            // Odd, so the center doesn't move at all
            (w / 2) * (h + 1) / 2
        } else {
            w * h / 4
        }
    }
}

pub struct Rot180;
impl Transform for Rot180 {
    const ORBITS: u64 = 1;

    fn applicable(allowed_families: AllowedTransformFamiles, _square: bool) -> bool {
        matches!(allowed_families, Rotate | RotateAndFlip)
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        if w % 2 == 1 && h % 2 == 1 {
            // Odd, so the center doesn't move at all
            (h / 2) * w + w / 2
        } else {
            w * h / 2
        }
    }
}

pub struct HFlip;
impl Transform for HFlip {
    const ORBITS: u64 = 1;

    fn applicable(allowed_families: AllowedTransformFamiles, _square: bool) -> bool {
        allowed_families == RotateAndFlip
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        h * (w / 2)
    }
}

pub struct VFlip;
impl Transform for VFlip {
    const ORBITS: u64 = 1;

    fn applicable(allowed_families: AllowedTransformFamiles, _square: bool) -> bool {
        allowed_families == RotateAndFlip
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        w * (h / 2)
    }
}

pub struct DFlip;
impl Transform for DFlip {
    const ORBITS: u64 = 2;

    fn applicable(allowed_families: AllowedTransformFamiles, square: bool) -> bool {
        allowed_families == RotateAndFlip && square
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        assert_eq!(w, h);
        w * (w - 1) / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rot90() {
        assert_eq!(Rot90::base_fixed(2, 2), 1);
        assert_eq!(Rot90::free(2, 2), 1);
        assert_eq!(Rot90::base_fixed(3, 3), 2);
        assert_eq!(Rot90::free(3, 3), 3);
        assert_eq!(Rot90::base_fixed(4, 4), 4);
        assert_eq!(Rot90::free(4, 4), 4);
        assert_eq!(Rot90::base_fixed(5, 5), 6);
        assert_eq!(Rot90::free(5, 5), 7);
    }

    #[test]
    fn test_rot180() {
        assert_eq!(Rot180::base_fixed(2, 2), 2);
        assert_eq!(Rot180::free(2, 2), 2);
        assert_eq!(Rot180::base_fixed(3, 2), 3);
        assert_eq!(Rot180::free(3, 2), 3);
        assert_eq!(Rot180::base_fixed(2, 3), 3);
        assert_eq!(Rot180::free(3, 2), 3);
        assert_eq!(Rot180::base_fixed(3, 3), 4);
        assert_eq!(Rot180::free(3, 3), 5);
        assert_eq!(Rot180::base_fixed(4, 4), 8);
        assert_eq!(Rot180::free(4, 4), 8);
    }

    #[test]
    fn test_hflip() {
        assert_eq!(HFlip::base_fixed(2, 2), 2);
        assert_eq!(HFlip::free(2, 2), 2);
        assert_eq!(HFlip::base_fixed(3, 2), 2);
        assert_eq!(HFlip::free(3, 2), 4);
        assert_eq!(HFlip::base_fixed(2, 3), 3);
        assert_eq!(HFlip::free(2, 3), 3);
        assert_eq!(HFlip::base_fixed(3, 3), 3);
        assert_eq!(HFlip::free(3, 3), 6);
        assert_eq!(HFlip::base_fixed(4, 4), 8);
        assert_eq!(HFlip::free(4, 4), 8);
    }

    #[test]
    fn test_vflip() {
        assert_eq!(VFlip::base_fixed(2, 2), 2);
        assert_eq!(VFlip::free(2, 2), 2);
        assert_eq!(VFlip::base_fixed(3, 2), 3);
        assert_eq!(VFlip::free(3, 2), 3);
        assert_eq!(VFlip::base_fixed(2, 3), 2);
        assert_eq!(VFlip::free(2, 3), 4);
        assert_eq!(VFlip::base_fixed(3, 3), 3);
        assert_eq!(VFlip::free(3, 3), 6);
        assert_eq!(VFlip::base_fixed(4, 4), 8);
        assert_eq!(VFlip::free(4, 4), 8);
    }

    #[test]
    fn test_dflip() {
        assert_eq!(DFlip::base_fixed(2, 2), 1);
        assert_eq!(DFlip::free(2, 2), 3);
        assert_eq!(DFlip::base_fixed(3, 3), 3);
        assert_eq!(DFlip::free(3, 3), 6);
        assert_eq!(DFlip::base_fixed(4, 4), 6);
        assert_eq!(DFlip::free(4, 4), 10);
    }
}
