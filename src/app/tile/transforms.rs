#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowedTransformFamiles {
    NoTransforms,
    Rotate,
    RotateAndFlip
}
use AllowedTransformFamiles::*;

pub trait Transform {
    const ORBITS: u64;
    fn applicable(allowed_families: AllowedTransformFamiles, square: bool) -> bool;
    fn base_fixed(w: u64, h: u64) -> u64;
    fn n_fixed(w: u64, h: u64, n_colours: u64) -> u64 {
        n_colours.pow(Self::base_fixed(w, h) as u32)
    }
    fn total_fixed(w: u64, h: u64, n_colours: u64) -> u64 {
        Self::n_fixed(w, h, n_colours) * Self::ORBITS
    }
}

pub struct NoXform;
impl Transform for NoXform {
    const ORBITS: u64 = 1;

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

    fn applicable(allowed_families: AllowedTransformFamiles, square: bool) -> bool {
        matches!(allowed_families, Rotate | RotateAndFlip) && square
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        assert_eq!(w, h);
        if w % 2 == 1 {
            // Odd, so the center doesn't move at all
            (w / 2) * (h + 1) / 2 + 1
        } else {
            w * h / 4
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rot90() {
        assert_eq!(Rot90::base_fixed(2, 2), 1);
        assert_eq!(Rot90::base_fixed(3, 3), 3);
        assert_eq!(Rot90::base_fixed(4, 4), 4);
        assert_eq!(Rot90::base_fixed(5, 5), 7);
    }

    #[test]
    fn test_rot180() {
        assert_eq!(Rot180::base_fixed(2, 2), 2);
        assert_eq!(Rot180::base_fixed(3, 2), 3);
        assert_eq!(Rot180::base_fixed(2, 3), 3);
        assert_eq!(Rot180::base_fixed(3, 3), 5);
        assert_eq!(Rot180::base_fixed(4, 4), 8);
        
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
            (h / 2) * w + w / 2 + 1
        } else {
            w * h  / 2
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
        
        h * (w / 2 + (w & 1))
    }

}


pub struct VFlip;
impl Transform for VFlip {
    const ORBITS: u64 = 1;

    fn applicable(allowed_families: AllowedTransformFamiles, _square: bool) -> bool {
        allowed_families == RotateAndFlip
    }

    fn base_fixed(w: u64, h: u64) -> u64 {
        w * (h / 2 + (h & 1))
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
        w * (w + 1) / 2
    }
}