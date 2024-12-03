use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{map, map_res, verify},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

enum Instr {
    Do,
    Dont,
    Mul(u32, u32),
}

fn parse(input: &str) -> Vec<Instr> {
    fn num(input: &str) -> IResult<&str, u32> {
        map_res(verify(digit1, |s: &str| s.len() <= 3), str::parse)(input)
    }
    let mul = delimited(tag("mul("), separated_pair(num, tag(","), num), tag(")"));
    let instr = alt((
        map(tag("do()"), |_| Instr::Do),
        map(tag("don't()"), |_| Instr::Dont),
        map(mul, |(a, b)| Instr::Mul(a, b)),
    ));
    let find = |p| map(many_till(take(1usize), p), |(_, g)| g);
    many1(find(instr))(input).unwrap().1
}

fn run<const P2: bool>(input: &str) -> u32 {
    let mut enabled = true;
    let mut sum = 0;
    for i in parse(input) {
        match i {
            Instr::Do => enabled = true,
            Instr::Dont => enabled = false,
            Instr::Mul(a, b) => {
                if enabled || !P2 {
                    sum += a * b
                }
            }
        }
    }
    sum
}

#[allow(non_upper_case_globals)]
pub const solve: fn(&str) -> u32 = run::<false>;

#[allow(non_upper_case_globals)]
pub const solve_2: fn(&str) -> u32 = run::<true>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            solve("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        )
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            solve_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        )
    }
}
