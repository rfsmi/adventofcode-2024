use core::str;
use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            map.insert((y as i32, x as i32), c);
        }
    }
    map
}

fn find_regions(map: HashMap<(i32, i32), char>) -> Vec<HashSet<(i32, i32)>> {
    let mut stack: Vec<_> = map.keys().copied().enumerate().collect();
    let mut seen = HashSet::new();
    let mut regions: HashMap<usize, HashSet<(i32, i32)>> = HashMap::new();
    while let Some((r, (y, x))) = stack.pop() {
        if !seen.insert((y, x)) {
            continue;
        }
        regions.entry(r).or_default().insert((y, x));
        let c = map.get(&(y, x));
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (y, x) = (y + dy, x + dx);
            if map.get(&(y, x)) == c {
                stack.push((r, (y, x)));
            }
        }
    }
    regions.into_values().collect()
}

fn count_perimeter(region: HashSet<(i32, i32)>) -> usize {
    region
        .iter()
        .map(|&(y, x)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(|(dy, dx)| (y + dy, x + dx))
                .filter(|p| !region.contains(p))
                .count()
        })
        .sum()
}

fn count_sides(mut region: HashSet<(i32, i32)>) -> usize {
    let mut sides = 0;
    for _ in 0..4 {
        region = region.into_iter().map(|(y, x)| (x, -y)).collect();
        let &min_y = region.iter().map(|(y, _)| y).min().unwrap();
        let &max_y = region.iter().map(|(y, _)| y).max().unwrap();
        let &min_x = region.iter().map(|(_, x)| x).min().unwrap();
        let &max_x = region.iter().map(|(_, x)| x).max().unwrap();
        for y in min_y..=max_y {
            let mut last = false;
            for x in min_x..=max_x {
                let upper = region.contains(&(y - 1, x));
                let lower = region.contains(&(y, x));
                if lower && !upper && !last {
                    sides += 1;
                }
                last = lower && !upper;
            }
        }
    }
    sides
}

pub fn solve(input: &str) -> usize {
    find_regions(parse(input))
        .into_iter()
        .map(|region| region.len() * count_perimeter(region))
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    find_regions(parse(input))
        .into_iter()
        .map(|region| region.len() * count_sides(region))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 1930)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 1206)
    }
}
