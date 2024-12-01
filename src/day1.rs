use std::iter::zip;

use itertools::Itertools;
use nom::{
    character::complete::{digit1, multispace0},
    combinator::{map, map_res},
    multi::many1,
    sequence::{pair, preceded},
    IResult,
};

fn num(n: &str) -> IResult<&str, u32> {
    preceded(multispace0, map_res(digit1, str::parse))(n)
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    map(many1(pair(num, num)), |pairs| pairs.into_iter().unzip())(input)
        .unwrap()
        .1
}

pub fn solve(input: &str) -> u32 {
    let (mut l, mut r) = parse(input);
    l.sort();
    r.sort();
    zip(l, r)
        .map(|(a, b)| (a as i64 - b as i64).abs() as u32)
        .sum()
}

pub fn solve_2(input: &str) -> u32 {
    let (l, r) = parse(input);
    let counts = r.into_iter().counts();
    l.into_iter()
        .map(move |n| n * *counts.get(&n).unwrap_or(&0) as u32)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            solve(
                "3   4
                4   3
                2   5
                1   3
                3   9
                3   3"
            ),
            11
        )
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            solve_2(
                "3   4
                4   3
                2   5
                1   3
                3   9
                3   3"
            ),
            31
        )
    }
}
