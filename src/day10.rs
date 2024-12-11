use std::collections::HashMap;

fn parse(input: &str) -> HashMap<(i32, i32), u32> {
    let mut map = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if let Some(n) = c.to_digit(10) {
                map.insert((y as i32, x as i32), n as u32);
            }
        }
    }
    map
}

fn find_trails(input: &str) -> HashMap<((i32, i32), (i32, i32)), usize> {
    let map = parse(input);
    let mut stack: Vec<_> = map
        .iter()
        .filter_map(|(&pos, &h)| (h == 0).then(|| (pos, pos, h)))
        .collect();
    let mut trails: HashMap<((i32, i32), (i32, i32)), usize> = HashMap::new();
    while let Some((start, (y, x), h)) = stack.pop() {
        if h == 9 {
            *trails.entry((start, (y, x))).or_default() += 1;
            continue;
        }
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (y, x) = (y + dy, x + dx);
            if map.get(&(y, x)) == Some(&(h + 1)) {
                stack.push((start, (y, x), h + 1));
            }
        }
    }
    trails
}

pub fn solve(input: &str) -> usize {
    find_trails(input).len()
}

pub fn solve_2(input: &str) -> usize {
    find_trails(input).values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 36)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 81)
    }
}
