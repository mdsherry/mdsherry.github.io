use std::collections::HashSet;

use bitvec::prelude::*;

use super::AllowedTransformFamiles;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Permutation {
    value: BitArr!(for 30, in Msb0, u32),
}

impl Permutation {
    pub fn new() -> Self {
        Self {
            value: bitarr![Msb0, u32; 0; 30],
        }
    }
    pub fn get(&self, n: u64) -> u8 {
        self.value[(n * 3) as usize..((n + 1) * 3) as usize].load()
    }
    pub fn set(&mut self, n: u64, value: u8) {
        self.value[(n * 3) as usize..((n + 1) * 3) as usize].store(value);
    }
    pub fn rotate_n(&self, n: u64, max: u64) -> Self {
        let mut new = Self::new();
        for i in 0..max {
            new.set((i + n) % max, self.get(i));
        }
        new
    }

    pub fn flip(&self, max: u64) -> Self {
        let mut new = Self::new();
        for i in 0..max {
            new.set(max - 1 - i, self.get(i));
        }
        new
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build_permutations(
    transforms: &Transforms,
    n_beads: u64,
    n_colours: u64,
    n: u64,
    mut permutation: Permutation,
    colour_counts: &mut [u64],
    colour_limit: u64,
    seen: &mut HashSet<Permutation>,
) {
    if n == n_beads {
        // We're done; canonicalize and add to `seen`
        seen.insert(transforms.canonicalize(&permutation));
        return;
    }

    for colour in 0..n_colours {
        permutation.set(n as u64, colour as u8);
        colour_counts[colour as usize] += 1;
        if colour_counts[colour as usize] <= colour_limit {
            build_permutations(
                transforms,
                n_beads,
                n_colours,
                n + 1,
                permutation.clone(),
                colour_counts,
                colour_limit,
                seen,
            );
        }
        colour_counts[colour as usize] -= 1;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transforms {
    n_beads: u64,
    allowed_families: AllowedTransformFamiles,
}

impl Transforms {
    pub fn new(n_beads: u64, allowed_families: AllowedTransformFamiles) -> Self {
        Transforms {
            n_beads,
            allowed_families,
        }
    }

    pub fn canonicalize(self, perm: &Permutation) -> Permutation {
        let n_beads = self.n_beads;
        let mut canonical = perm.clone();
        if matches!(
            self.allowed_families,
            AllowedTransformFamiles::Rotate | AllowedTransformFamiles::RotateAndFlip
        ) {
            for n in 1..n_beads {
                canonical = canonical.min(perm.rotate_n(n, n_beads));
            }
        }
        if matches!(
            self.allowed_families,
            AllowedTransformFamiles::RotateAndFlip
        ) {
            let flipped = perm.flip(n_beads);
            canonical = canonical.min(flipped.clone());
            for n in 1..n_beads {
                canonical = canonical.min(flipped.rotate_n(n, n_beads));
            }
        }
        canonical
    }
}
