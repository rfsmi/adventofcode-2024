use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::map_res,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};

trait Combos {
    fn combos(a: i64, b: i64) -> impl Iterator<Item = i64>;
}

impl Combos for Problem<true> {
    fn combos(a: i64, b: i64) -> impl Iterator<Item = i64> {
        [
            a * b,
            a + b,
            (a.to_string() + &b.to_string()).parse().unwrap(),
        ]
        .into_iter()
    }
}

impl Combos for Problem<false> {
    fn combos(a: i64, b: i64) -> impl Iterator<Item = i64> {
        [a * b, a + b].into_iter()
    }
}

struct Problem<const P2: bool> {
    target: i64,
    nums: Vec<i64>,
}

impl<const P2: bool> Problem<P2>
where
    Problem<P2>: Combos,
{
    fn solveable(&self, a: i64, i: usize) -> bool {
        if a > self.target {
            return false;
        }
        let Some(&b) = self.nums.get(i) else {
            return a == self.target;
        };
        Self::combos(a, b).any(|c| self.solveable(c, i + 1))
    }
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    fn num(input: &str) -> IResult<&str, i64> {
        map_res(digit1, str::parse)(input)
    }
    many1(preceded(
        multispace0,
        separated_pair(num, tag(":"), many1(preceded(space0, num))),
    ))(input)
    .unwrap()
    .1
}

pub fn solve(input: &str) -> i64 {
    let eqs = parse(input);
    eqs.into_iter()
        .map(|(target, nums)| Problem::<false> { target, nums })
        .filter(|problem| problem.solveable(0, 0))
        .map(|problem| problem.target)
        .sum()
}

pub fn solve_2(input: &str) -> i64 {
    let eqs = parse(input);
    eqs.into_iter()
        .map(|(target, nums)| Problem::<true> { target, nums })
        .filter(|problem| problem.solveable(0, 0))
        .map(|problem| problem.target)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 3749)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 11387)
    }
}
