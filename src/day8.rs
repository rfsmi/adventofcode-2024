use std::collections::HashMap;

use itertools::Itertools;

fn parse(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            map.entry(c).or_default().push((y as i32, x as i32));
        }
    }
    map
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }
    a
}

fn compute<Gen, Antinodes, Steps>(input: &str, gen_antinodes: Gen) -> usize
where
    Gen: Copy + Fn(((i32, i32), (i32, i32))) -> Antinodes,
    Antinodes: 'static + IntoIterator<Item = ((i32, i32), (i32, i32), Steps)>,
    Steps: IntoIterator<Item = i32>,
{
    let map = parse(input);
    let &(max_y, max_x) = map.values().flatten().max().unwrap();
    let in_bounds = move |(y, x): &(i32, i32)| (0..=max_y).contains(y) && (0..=max_x).contains(x);
    map.into_iter()
        .filter(|&(c, _)| c != '.')
        .flat_map(move |(_, coords)| {
            coords
                .into_iter()
                .tuple_combinations()
                .flat_map(gen_antinodes)
        })
        .flat_map(|((y, x), (dy, dx), range)| {
            range
                .into_iter()
                .map(move |i| (y + i * dy, x + i * dx))
                .take_while(in_bounds)
        })
        .unique()
        .count()
}

pub fn solve(input: &str) -> usize {
    compute(input, |((y1, x1), (y2, x2))| {
        let (dy, dx) = (y2 - y1, x2 - x1);
        [((y1, x1), (dy, dx), [-1]), ((y2, x2), (dy, dx), [1])]
    })
}

pub fn solve_2(input: &str) -> usize {
    compute(input, |((y1, x1), (y2, x2))| {
        let (dy, dx) = (y2 - y1, x2 - x1);
        let div = gcd(dy, dx);
        [
            ((y1, x1), (dy / div, dx / div), 0..),
            ((y1, x1), (-dy / div, -dx / div), 1..),
        ]
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 14)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 34)
    }
}
