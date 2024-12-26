use std::{collections::HashMap, hash::Hash, iter::once};

use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.lines().filter_map(|l| l.trim().parse().ok())
}

fn simulate(mut num: u64, count: usize) -> impl Iterator<Item = u64> {
    let mix = |a: u64, b: u64| (a ^ b) % 16777216;
    once(num).chain((0..count).map(move |_| {
        num = mix(num, num * 64);
        num = mix(num, num / 32);
        num = mix(num, num * 2048);
        num
    }))
}

pub fn solve(input: &str) -> u64 {
    parse(input)
        .filter_map(|num| simulate(num, 2000).last())
        .sum()
}

trait Combine<K, V> {
    fn combine(self, f: impl Fn(V, V) -> V) -> HashMap<K, V>;
}

impl<T, K, V> Combine<K, V> for T
where
    T: IntoIterator<Item = (K, V)>,
    K: Hash + Eq,
    V: Copy,
{
    fn combine(self, f: impl Fn(V, V) -> V) -> HashMap<K, V> {
        let mut map = HashMap::new();
        for (k, v) in self {
            map.entry(k)
                .and_modify(|old_v: &mut V| *old_v = f(*old_v, v))
                .or_insert(v);
        }
        map
    }
}

pub fn solve_2(input: &str) -> u64 {
    parse(input)
        .flat_map(|num| {
            simulate(num, 2000)
                .map(|n| n % 10)
                .tuple_windows()
                .map(|(a, b)| (b as i8 - a as i8, b))
                .tuple_windows()
                .map(|((a, _), (b, _), (c, _), (d, n))| ((a, b, c, d), n))
                .combine(|n, _| n)
        })
        .combine(u64::saturating_add)
        .into_values()
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            solve(
                "1
                10
                100
                2024"
            ),
            37327623
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            solve_2(
                "1
                2
                3
                2024"
            ),
            23
        );
    }
}
