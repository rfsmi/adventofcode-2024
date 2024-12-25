use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::{chain, Itertools};

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

fn parse(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(str::trim)
        .filter_map(|l| l.strip_suffix("A"))
}

struct Keypad {
    dpad: HashMap<(i32, i32), char>,
}

impl Keypad {
    fn new() -> Self {
        #[rustfmt::skip]
        let dpad = make_keypad(&[
            [' ', '^', 'A'],
            ['<', 'v', '>'],
        ]);
        Self { dpad }
    }

    fn move_pad(
        &self,
        memo: &mut HashMap<((i32, i32), (i32, i32), u32), u64>,
        keypad: &HashMap<(i32, i32), char>,
        start: (i32, i32),
        end: (i32, i32),
        n_robots: u32,
    ) -> u64 {
        if end == start || n_robots == 0 {
            return 1;
        }
        if let Some(&cost) = memo.get(&(start, end, n_robots)) {
            return cost;
        }
        let (&accept, _) = self.dpad.iter().find(|&(_, v)| v == &'A').unwrap();
        let mut queue: BinaryHeap<_> = [Reverse((0, accept, start))].into();
        let mut seen = HashSet::new();
        while let Some(Reverse((cost, dpad, child))) = queue.pop() {
            if !seen.insert((dpad, child)) {
                continue;
            }
            if (dpad, child) == (accept, end) {
                memo.insert((start, end, n_robots), cost);
                return cost;
            }
            for (next_dpad, dir) in self.dpad.clone() {
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
                    let d_cost = self.move_pad(memo, &self.dpad, dpad, next_dpad, n_robots - 1);
                    queue.push(Reverse((cost + d_cost, next_dpad, next_child)));
                }
            }
        }
        panic!()
    }

    fn solve(&self, code: &str, n_robots: u32) -> u64 {
        let keypad = make_keypad(&[
            ['7', '8', '9'],
            ['4', '5', '6'],
            ['1', '2', '3'],
            [' ', '0', 'A'],
        ]);
        let mut memo: HashMap<((i32, i32), (i32, i32), u32), u64> = Default::default();
        chain![['A'], code.chars(), ['A']]
            .tuple_windows()
            .map(|(start, end)| {
                let (&start, _) = keypad.iter().find(|&(_, v)| v == &start).unwrap();
                let (&end, _) = keypad.iter().find(|&(_, v)| v == &end).unwrap();
                self.move_pad(&mut memo, &keypad, start, end, n_robots)
            })
            .sum()
    }
}

pub fn solve(input: &str) -> u64 {
    let keypad = Keypad::new();
    parse(input)
        .map(|code| keypad.solve(code, 3) * code.parse::<u64>().unwrap())
        .sum()
}

pub fn solve_2(input: &str) -> u64 {
    let keypad = Keypad::new();
    parse(input)
        .map(|code| keypad.solve(code, 26) * code.parse::<u64>().unwrap())
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
