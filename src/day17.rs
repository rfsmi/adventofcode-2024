use std::{collections::VecDeque, iter::from_fn};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::map_res,
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};

#[derive(Clone)]
struct VM {
    ip: usize,
    reg: [u64; 3],
    mem: Vec<u64>,
}

impl VM {
    fn new(input: &str) -> Self {
        fn num(input: &str) -> IResult<&str, u64> {
            preceded(space0, map_res(digit1, str::parse))(input)
        }
        let (_, (a, b, c, mem)) = tuple((
            preceded(pair(multispace0, tag("Register A:")), num),
            preceded(pair(multispace0, tag("Register B:")), num),
            preceded(pair(multispace0, tag("Register C:")), num),
            preceded(
                pair(multispace0, tag("Program:")),
                separated_list1(tag(","), num),
            ),
        ))(input)
        .unwrap();
        Self {
            ip: 0,
            reg: [a, b, c],
            mem,
        }
    }

    fn last_combo(&mut self) -> u64 {
        match *self.mem.get(self.ip - 1).unwrap() {
            i @ 0..=3 => i,
            i @ 4..=6 => self.reg[i as usize - 4],
            _ => panic!(),
        }
    }

    fn last_literal(&mut self) -> u64 {
        *self.mem.get(self.ip - 1).unwrap()
    }

    fn with_a(mut self, a: u64) -> Self {
        self.reg[0] = a;
        self
    }

    fn run(mut self) -> impl Iterator<Item = u64> {
        from_fn(move || loop {
            let Some(&op) = self.mem.get(self.ip) else {
                return None;
            };
            self.ip += 2;
            match op {
                1 => self.reg[1] ^= self.last_literal(),
                2 => self.reg[1] = self.last_combo() & 0x07,
                3 if self.reg[0] == 0 => (),
                3 => self.ip = self.last_literal() as usize,
                4 => self.reg[1] ^= self.reg[2],
                5 => return Some(self.last_combo() & 0x07),
                6 => self.reg[1] = self.reg[0] >> self.last_combo(),
                7 => self.reg[2] = self.reg[0] >> self.last_combo(),
                0 => self.reg[0] = self.reg[0] >> self.last_combo(),
                _ => panic!(),
            }
        })
    }
}

pub fn solve(input: &str) -> String {
    VM::new(input).run().join(",")
}

pub fn solve_2(input: &str) -> u64 {
    let vm = VM::new(input);
    let mut queue: VecDeque<(u64, &[u64])> = [(0, &vm.mem[..])].into();
    while let Some((a, mem)) = queue.pop_front() {
        let [rest @ .., targ] = mem else {
            return a;
        };
        for n in 0..8 {
            let a = 8 * a + n;
            if vm.clone().with_a(a).run().next() == Some(*targ) {
                queue.push_back((a, rest));
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }
}
