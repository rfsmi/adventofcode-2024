use std::collections::HashMap;

use itertools::iproduct;

fn parse(input: &str) -> (HashMap<(isize, isize), char>, isize, isize) {
    let mut result = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            result.insert((y as isize, x as isize), c);
        }
    }
    let width = result.keys().map(|(_, x)| x + 1).max().unwrap();
    let height = result.keys().map(|(y, _)| y + 1).max().unwrap();
    (result, height, width)
}

fn contains(
    grid: &HashMap<(isize, isize), char>,
    needle: &str,
    (mut y, mut x, dy, dx): (isize, isize, isize, isize),
) -> bool {
    for c in needle.chars() {
        if grid.get(&(y, x)) != Some(&c) {
            return false;
        }
        y += dy;
        x += dx;
    }
    true
}

pub fn solve(input: &str) -> usize {
    let (grid, height, width) = parse(input);
    iproduct!(0..height, 0..width, [-1, 0, 1], [-1, 0, 1])
        .filter(|&p| contains(&grid, "XMAS", p))
        .count()
}

pub fn solve_2(input: &str) -> usize {
    let (grid, height, width) = parse(input);
    iproduct!(0..height, 0..width, ["MAS", "SAM"], ["MAS", "SAM"])
        .filter(|(y, x, n1, n2)| {
            contains(&grid, n1, (y - 1, x - 1, 1, 1)) && contains(&grid, n2, (y - 1, x + 1, 1, -1))
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE: &'static str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 18)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 9)
    }
}
