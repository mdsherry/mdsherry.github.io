pub fn simple_count(draws: u64, n_colours: u64, each_count: u64) -> u64 {
    count(draws, &mut vec![each_count as u8; n_colours as usize])
}

fn count(draws: u64, left: &mut [u8]) -> u64 {
    if draws == 0 {
        return 1;
    }
    let mut total = 0;
    for i in 0..left.len() {
        if left[i] > 0 {
            left[i] -= 1;
            total += count(draws - 1, left);
            left[i] += 1;
        }
    }
    total
}

pub fn count2(draws: u64, left_counts: &mut [u64]) -> u64 {
    if draws == 0 {
        return 1;
    }
    let mut total = 0;
    for i in 1..left_counts.len() {
        if left_counts[i] > 0 {
            let count = left_counts[i];
            left_counts[i] -= 1;
            left_counts[i - 1] += 1;
            total += count * count2(draws - 1, left_counts);
            left_counts[i - 1] -= 1;
            left_counts[i] += 1;
        }
    }
    total
}

pub fn count3(draws: &[u8], left_counts: &mut [u64]) -> u64 {
    if let Some((next_draw, draws)) = draws.split_first() {
        let next_draw = *next_draw as usize;
        let mut total = 0;
        for i in next_draw..left_counts.len() {
            if left_counts[i] > 0 {
                let count = left_counts[i];
                left_counts[i] -= 1;
                left_counts[i - next_draw] += 1;
                total += count * count3(draws, left_counts);
                left_counts[i - next_draw] -= 1;
                left_counts[i] += 1;
            }
        }
        total
    } else {
        1
    }
}

// fn count2(draws: u64, n_colours: u64, each_count: u64) {
//     let mut total = 1;
//     for i in 0..n_colours {

//     }
// }

// fn fact(n: u64) -> u64 {
//     (1..=n).product()
// }

#[cfg(test)]
mod test {
    use super::{count, count2};
    fn choose(n: u64, k: u64) -> u64 {
        (n - k + 1..=n).product()
    }
    #[test]
    fn test() {
        assert_eq!(1, count(1, &mut [1, 0, 0]));
        assert_eq!(2, count(1, &mut [1, 0, 1]));
        assert_eq!(3, count(1, &mut [1, 1, 1]));
        assert_eq!(1, count(1, &mut [2, 0, 0]));
        assert_eq!(1, count(2, &mut [2, 0, 0]));
        assert_eq!(0, count(3, &mut [2, 0, 0]));
        assert_eq!(2, count(1, &mut [2, 1, 0]));
        assert_eq!(3, count(2, &mut [2, 1, 0]));
        assert_eq!(3, count(3, &mut [2, 1, 0]));
        assert_eq!(90, count(5, &mut [2, 2, 2]));
        assert_eq!(choose(4, 2), count(2, &mut [1, 1, 1, 1]));
        assert_eq!(24, count(3, &mut [1, 1, 1, 1]));
    }
    #[test]
    fn test2() {
        assert_eq!(1, count2(1, &mut [0, 1, 0]));
        assert_eq!(2, count2(1, &mut [0, 2, 0]));
        assert_eq!(3, count2(1, &mut [0, 3, 0]));
        assert_eq!(1, count2(1, &mut [0, 0, 1]));
        assert_eq!(1, count2(2, &mut [0, 0, 1]));
        assert_eq!(0, count2(3, &mut [0, 0, 1]));
        assert_eq!(2, count2(1, &mut [0, 1, 1]));
        assert_eq!(3, count2(2, &mut [0, 1, 1]));
        assert_eq!(3, count2(3, &mut [0, 1, 1]));
        assert_eq!(90, count2(5, &mut [0, 0, 3]));
        assert_eq!(2, count2(1, &mut [0, 2]));
    }
}
