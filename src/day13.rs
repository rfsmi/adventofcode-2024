use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::many1,
    sequence::{pair, preceded, tuple},
    IResult,
};

fn parse(input: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    fn num(input: &str) -> IResult<&str, i64> {
        map_res(digit1, str::parse)(input)
    }
    let button_a = pair(
        preceded(tag("Button A: X+"), num),
        preceded(tag(", Y+"), num),
    );
    let button_b = pair(
        preceded(tag("Button B: X+"), num),
        preceded(tag(", Y+"), num),
    );
    let prize = pair(preceded(tag("Prize: X="), num), preceded(tag(", Y="), num));
    many1(tuple((
        preceded(multispace0, button_a),
        preceded(multispace0, button_b),
        preceded(multispace0, prize),
    )))(input)
    .unwrap()
    .1
}

trait Compute {
    fn compute(self) -> i64;
}

impl<T> Compute for T
where
    T: Iterator<Item = ((i64, i64), (i64, i64), (i64, i64))>,
{
    fn compute(self) -> i64 {
        self.filter_map(|(a, b, p)| {
            let s_num: i64 = p.0 * a.1 - p.1 * a.0;
            let s_denom = b.0 * a.1 - b.1 * a.0;
            if s_num % s_denom != 0 {
                return None;
            }
            let s = s_num / s_denom;
            let t_num = p.1 - s * b.1;
            if t_num % a.1 != 0 {
                return None;
            }
            let t = t_num / a.1;
            Some((s, t))
        })
        .map(|(s, t)| s + 3 * t)
        .sum()
    }
}

pub fn solve(input: &str) -> i64 {
    parse(input).into_iter().compute()
}

pub fn solve_2(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|(a, b, p)| {
            let p = (p.0 + 10000000000000, p.1 + 10000000000000);
            (a, b, p)
        })
        .compute()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 480)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 875318608908)
    }
}
