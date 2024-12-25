use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeBounds,
};

use itertools::iproduct;

type Pos = (i32, i32);

fn parse(input: &str) -> (HashSet<Pos>, Pos) {
    let mut map = HashSet::new();
    let mut end = (0, 0);
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let pos = (y as i32, x as i32);
            match c {
                'E' => end = pos,
                '#' => continue,
                _ => (),
            }
            map.insert(pos);
        }
    }
    (map, end)
}

fn cheats(
    max_r: i32,
    dists: &HashMap<(i32, i32), i32>,
    (y, x): Pos,
    d1: i32,
) -> impl Iterator<Item = i32> + '_ {
    iproduct!(-max_r..=max_r, -max_r..=max_r)
        .filter_map(move |(dy, dx)| {
            let r = dy.abs() + dx.abs();
            (r <= max_r).then_some((dy, dx, r))
        })
        .filter_map(move |(dy, dx, r)| dists.get(&(y + dy, x + dx)).map(|&d2| d1 - d2 - r))
}

pub fn compute(max_cheat: i32, savings_range: impl RangeBounds<i32>, input: &str) -> usize {
    let (map, end) = parse(input);
    let mut queue: VecDeque<_> = [(0, end)].into();
    let mut dists = HashMap::new();
    while let Some((dist, pos @ (y, x))) = queue.pop_front() {
        if !map.contains(&pos) || dists.contains_key(&pos) {
            continue;
        }
        dists.insert(pos, dist);
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (y, x) = (y + dy, x + dx);
            queue.push_back((dist + 1, (y, x)));
        }
    }
    dists
        .iter()
        .flat_map(|(&p1, &d1)| cheats(max_cheat, &dists, p1, d1))
        .filter(|saving| savings_range.contains(saving))
        .count()
}

pub fn solve(input: &str) -> usize {
    compute(2, 100.., input)
}

pub fn solve_2(input: &str) -> usize {
    compute(20, 100.., input)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    ";

    #[test]
    fn test_sample() {
        assert_eq!(compute(2, 2..=2, SAMPLE), 14);
    }
}
