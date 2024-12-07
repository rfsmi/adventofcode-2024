use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let mut result = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            result.insert((y as i32, x as i32), c);
        }
    }
    result
}

fn escapes(
    grid: &HashMap<(i32, i32), char>,
    mut pos: (i32, i32),
    mut dir: (i32, i32),
) -> Option<usize> {
    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    while visited.insert((pos, dir)) {
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        let Some(&c) = grid.get(&next_pos) else {
            return Some(visited.into_iter().unique_by(|&(pos, _)| pos).count());
        };
        if c == '#' {
            dir = (dir.1, -dir.0);
        } else {
            pos = next_pos;
        }
    }
    None
}

pub fn solve(input: &str) -> usize {
    let grid = parse(input);
    let pos = grid
        .iter()
        .find_map(|(&pos, &c)| if c == '^' { Some(pos) } else { None })
        .unwrap();
    escapes(&grid, pos, (-1, 0)).unwrap()
}

pub fn solve_2(input: &str) -> usize {
    let grid = parse(input);
    let start = grid
        .iter()
        .find_map(|(&pos, &c)| if c == '^' { Some(pos) } else { None })
        .unwrap();
    let &min = grid.keys().min().unwrap();
    let &max = grid.keys().max().unwrap();
    iproduct!(min.0 - 1..=max.0 + 1, min.1 - 1..=max.1 + 1)
        .filter(|&pos| pos != start)
        .filter(|&pos| {
            let mut grid = grid.clone();
            grid.insert(pos, '#');
            escapes(&grid, start, (-1, 0)).is_none()
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 41)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 6)
    }
}
