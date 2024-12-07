use std::{
    collections::{HashMap, HashSet},
    iter::from_fn,
};

use itertools::Itertools;

fn parse(input: &str) -> (HashMap<(i32, i32), char>, (i32, i32)) {
    let mut start = None;
    let mut result = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let pos = (y as i32, x as i32);
            result.insert(pos, c);
            if c == '^' {
                start = Some(pos);
            }
        }
    }
    (result, start.unwrap())
}

fn walk(
    grid: &HashMap<(i32, i32), char>,
    mut pos: (i32, i32),
    mut dir: (i32, i32),
) -> impl Iterator<Item = ((i32, i32), (i32, i32))> + '_ {
    from_fn(move || {
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if *grid.get(&next_pos)? == '#' {
            dir = (dir.1, -dir.0);
        } else {
            pos = next_pos;
        }
        Some((pos, dir))
    })
    .fuse()
}

pub fn solve(input: &str) -> usize {
    let (grid, pos) = parse(input);
    walk(&grid, pos, (-1, 0)).unique_by(|&(pos, _)| pos).count()
}

pub fn solve_2(input: &str) -> usize {
    let (grid, start) = parse(input);
    let dir = (-1, 0);
    let mut visited_dirs: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut result = 0;
    for (pos, dir) in walk(&grid, start, dir) {
        visited_dirs.insert((pos, dir));
        visited.insert(pos);
        let block_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if visited.contains(&block_pos) || matches!(grid.get(&block_pos), Some('#' | '^')) {
            continue;
        }
        let mut grid = grid.clone();
        grid.insert(block_pos, '#');
        let mut visited_dirs = visited_dirs.clone();
        if walk(&grid, pos, (dir.1, -dir.0)).any(|k| !visited_dirs.insert(k)) {
            result += 1;
        }
    }
    result
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
