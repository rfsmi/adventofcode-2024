use core::str;
use std::collections::HashMap;

use itertools::Itertools;

fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_ascii_whitespace()
        .filter_map(|n| str::parse(n).ok())
        .collect()
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }
    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (s1, s2) = stone_str.split_at(stone_str.len() / 2);
        let (s1, s2) = (s1.parse().unwrap(), s2.parse().unwrap());
        return vec![s1, s2];
    }
    vec![stone * 2024]
}

pub fn compute<const N: usize>(input: &str) -> usize {
    let mut memo = HashMap::new();
    let mut counts = parse(input).into_iter().counts();
    for _ in 0..N {
        let mut new_counts = HashMap::new();
        for (stone, n) in counts {
            for stone in memo.entry(stone).or_insert_with(|| blink(stone)) {
                *new_counts.entry(*stone).or_default() += n;
            }
        }
        counts = new_counts;
    }
    counts.values().sum()
}

pub fn solve(input: &str) -> usize {
    compute::<25>(input)
}

pub fn solve_2(input: &str) -> usize {
    compute::<75>(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(solve("125 17"), 55312)
    }
}
