use std::collections::HashSet;

use bitvec::prelude::*;

use super::transforms::AllowedTransformFamiles;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Permutation {
    value: BitArr!(for 75, in u32, Msb0),
}

impl Permutation {
    pub fn new() -> Self {
        Self {
            value: bitarr![u32, Msb0; 0; 75],
        }
    }
    pub fn get(&self, x: usize, y: usize, w: usize) -> u8 {
        let offset = y * w + x;
        self.value[offset * 3..(offset + 1) * 3].load()
    }
    pub fn set(&mut self, x: usize, y: usize, w: usize, value: u8) {
        let offset = y * w + x;
        self.value[offset * 3..(offset + 1) * 3].store(value);
    }
    pub fn rotate_90(&self, w: usize, h: usize) -> Self {
        assert_eq!(w, h);
        let mut new = Self::new();
        for x in 0..w {
            for y in 0..h {
                new.set(y, w - 1 - x, w, self.get(x, y, w));
            }
        }
        new
    }

    pub fn rotate_180(&self, w: usize, h: usize) -> Self {
        let mut new = Self::new();
        for x in 0..w {
            for y in 0..h {
                new.set(w - 1 - x, h - 1 - y, w, self.get(x, y, w));
            }
        }
        new
    }

    pub fn rotate_270(&self, w: usize, h: usize) -> Self {
        assert_eq!(w, h);
        let mut new = Self::new();
        for x in 0..w {
            for y in 0..h {
                new.set(h - 1 - y, x, w, self.get(x, y, w));
            }
        }
        new
    }

    pub fn hflip(&self, w: usize, h: usize) -> Self {
        let mut new = Self::new();
        for x in 0..w {
            for y in 0..h {
                new.set(w - 1 - x, y, w, self.get(x, y, w));
            }
        }
        new
    }

    pub fn vflip(&self, w: usize, h: usize) -> Self {
        let mut new = Self::new();
        for x in 0..w {
            for y in 0..h {
                new.set(x, h - 1 - y, w, self.get(x, y, w));
            }
        }
        new
    }

    pub fn dflip1(&self, w: usize, h: usize) -> Self {
        assert_eq!(w, h);
        let mut new = Self::new();
        for x in 0..w {
            for y in 0..h {
                new.set(y, x, w, self.get(x, y, w));
            }
        }
        new
    }

    pub fn dflip2(&self, w: usize, h: usize) -> Self {
        assert_eq!(w, h);
        let mut new = Self::new();
        for x in 0..w {
            for y in 0..h {
                new.set(h - 1 - y, w - 1 - x, w, self.get(x, y, w));
            }
        }
        new
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_square_rotation() {
        let mut perm = Permutation::new();
        perm.set(0, 0, 2, 1);
        perm.set(1, 0, 2, 2);
        perm.set(0, 1, 2, 3);
        perm.set(1, 1, 2, 4);
        let rot_90 = perm.rotate_90(2, 2);
        let rot_270 = perm.rotate_270(2, 2);
        let identity = rot_90.rotate_270(2, 2);
        assert_eq!(perm, identity);
        let identity = rot_270.rotate_90(2, 2);
        assert_eq!(perm, identity);
        let rot_90_90 = rot_90.rotate_90(2, 2);
        let rot_180 = perm.rotate_180(2, 2);
        assert_eq!(rot_90_90, rot_180);
        let transform = Transforms::new(2, 2, AllowedTransformFamiles::Rotate);
        assert_eq!(
            transform.canonicalize(&perm),
            transform.canonicalize(&rot_180)
        );
        assert_eq!(1, perm.get(0, 0, 2));
        assert_eq!(2, perm.get(1, 0, 2));
        assert_eq!(3, perm.get(0, 1, 2));
        assert_eq!(4, perm.get(1, 1, 2));
    }

    #[test]
    fn test_rect_rotation() {
        let mut perm = Permutation::new();
        perm.set(0, 0, 3, 1);
        perm.set(1, 0, 3, 2);
        perm.set(2, 0, 3, 3);
        perm.set(0, 1, 3, 4);
        perm.set(1, 1, 3, 5);
        perm.set(2, 1, 3, 6);
        let rot_180 = perm.rotate_180(3, 2);
        let identity = rot_180.rotate_180(3, 2);
        assert_eq!(perm, identity);
        assert_eq!(perm.get(0, 0, 3), 1);
        assert_eq!(perm.get(1, 0, 3), 2);
        assert_eq!(perm.get(2, 0, 3), 3);
        assert_eq!(perm.get(0, 1, 3), 4);
        assert_eq!(perm.get(1, 1, 3), 5);
        assert_eq!(perm.get(2, 1, 3), 6);
        let transform = Transforms::new(3, 2, AllowedTransformFamiles::Rotate);
        assert_eq!(
            transform.canonicalize(&perm),
            transform.canonicalize(&rot_180)
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_permutations(
    transforms: &Transforms,
    w: u64,
    h: u64,
    n_colours: u64,
    mut x: u64,
    mut y: u64,
    colour_counts: &mut [u64],
    colour_limit: u64,
    mut permutation: Permutation,
    seen: &mut HashSet<Permutation>,
) {
    if x == w {
        x = 0;
        y += 1;
        if y == h {
            // We're done; canonicalize and add to `seen`
            seen.insert(transforms.canonicalize(&permutation));
            return;
        }
    }

    if x >= w || y >= h {
        panic!("Out of bounds! {}/{} {}/{}", x, w, y, h);
    }
    for colour in 0..n_colours {
        colour_counts[colour as usize] += 1;
        if colour_counts[colour as usize] <= colour_limit {
            permutation.set(x as usize, y as usize, w as usize, colour as u8);
            build_permutations(
                transforms,
                w,
                h,
                n_colours,
                x + 1,
                y,
                colour_counts,
                colour_limit,
                permutation.clone(),
                seen,
            );
        }
        colour_counts[colour as usize] -= 1;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transforms {
    w: u64,
    h: u64,
    allowed_families: AllowedTransformFamiles,
}

impl Transforms {
    pub fn new(w: u64, h: u64, allowed_families: AllowedTransformFamiles) -> Self {
        Transforms {
            w,
            h,
            allowed_families,
        }
    }

    pub fn canonicalize(self, perm: &Permutation) -> Permutation {
        let square = self.w == self.h;
        let w = self.w as usize;
        let h = self.h as usize;
        let mut canonical = perm.clone();
        if matches!(
            self.allowed_families,
            AllowedTransformFamiles::Rotate | AllowedTransformFamiles::RotateAndFlip
        ) {
            if square {
                canonical = canonical
                    .min(perm.rotate_90(w, h))
                    .min(perm.rotate_270(w, h));
            }
            canonical = canonical.min(perm.rotate_180(w, h));
        }
        if matches!(
            self.allowed_families,
            AllowedTransformFamiles::RotateAndFlip
        ) {
            if square {
                canonical = canonical.min(perm.dflip1(w, h)).min(perm.dflip2(w, h));
            }
            canonical = canonical.min(perm.hflip(w, h));
            canonical = canonical.min(perm.vflip(w, h));
        }
        canonical
    }
}
