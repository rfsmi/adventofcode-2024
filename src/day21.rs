use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    iter::repeat_n,
};

use itertools::{chain, Itertools};

fn parse(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter_map(|l| l.strip_suffix("A"))
}

fn make_keypad<const M: usize, const N: usize>(
    chars: &[[char; N]; M],
) -> HashMap<(i32, i32), char> {
    let mut result = HashMap::new();
    for (y, row) in chars.into_iter().enumerate() {
        for (x, &c) in row.into_iter().enumerate() {
            if c != ' ' {
                result.insert((y as i32, x as i32), c);
            }
        }
    }
    result
}

fn calc_move_cost(
    memo: &mut HashMap<((i32, i32), (i32, i32), usize), u64>,
    pads: &[&HashMap<(i32, i32), char>],
    start: (i32, i32),
    end: (i32, i32),
) -> u64 {
    let Some((&keypad, &dpad)) = pads.into_iter().next_tuple() else {
        return 1;
    };
    if end == start {
        return 1;
    }
    if let Some(&cost) = memo.get(&(start, end, pads.len())) {
        return cost;
    }
    let (&accept, _) = dpad.iter().find(|&(_, v)| v == &'A').unwrap();
    let mut queue: BinaryHeap<_> = [Reverse((0, accept, start))].into();
    let mut seen = HashSet::new();
    while let Some(Reverse((cost, parent, child))) = queue.pop() {
        if !seen.insert((parent, child)) {
            continue;
        }
        if (parent, child) == (accept, end) {
            memo.insert((start, end, pads.len()), cost);
            return cost;
        }
        for (&next_parent, &dir) in dpad.iter() {
            let (dy, dx) = match dir {
                '^' => (-1, 0),
                '>' => (0, 1),
                'v' => (1, 0),
                '<' => (0, -1),
                'A' => (0, 0),
                _ => panic!(),
            };
            let next_child = (child.0 + dy, child.1 + dx);
            if keypad.contains_key(&next_child) {
                let d_cost = calc_move_cost(memo, &pads[1..], parent, next_parent);
                queue.push(Reverse((cost + d_cost, next_parent, next_child)));
            }
        }
    }
    panic!()
}

fn compute(
    memo: &mut HashMap<((i32, i32), (i32, i32), usize), u64>,
    n_robots: usize,
    code: &str,
) -> u64 {
    #[rustfmt::skip]
    let dpad = make_keypad(&[
        [' ', '^', 'A'],
        ['<', 'v', '>'],
    ]);
    let keypad = make_keypad(&[
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ]);
    let pads: Vec<_> = chain!([&keypad], repeat_n(&dpad, n_robots)).collect();
    chain![['A'], code.chars(), ['A']]
        .tuple_windows()
        .map(|(start, end)| {
            let (&start, _) = keypad.iter().find(|&(_, v)| v == &start).unwrap();
            let (&end, _) = keypad.iter().find(|&(_, v)| v == &end).unwrap();
            calc_move_cost(memo, &pads[..], start, end)
        })
        .sum()
}

pub fn solve(input: &str) -> u64 {
    let mut memo = HashMap::new();
    parse(input)
        .map(|code| compute(&mut memo, 3, code) * code.parse::<u64>().unwrap())
        .sum()
}

pub fn solve_2(input: &str) -> u64 {
    let mut memo = HashMap::new();
    parse(input)
        .map(|code| compute(&mut memo, 26, code) * code.parse::<u64>().unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        029A
        980A
        179A
        456A
        379A
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 126384);
    }
}
