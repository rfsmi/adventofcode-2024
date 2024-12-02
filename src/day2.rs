use itertools::Itertools;
use nom::{
    character::complete::{digit1, multispace0, space0},
    combinator::map_res,
    multi::many1,
    sequence::preceded,
    IResult,
};

fn parse(input: &str) -> Vec<Vec<u32>> {
    fn num(input: &str) -> IResult<&str, u32> {
        preceded(space0, map_res(digit1, str::parse))(input)
    }
    many1(preceded(multispace0, many1(num)))(input).unwrap().1
}

fn is_safe(report: &[u32]) -> bool {
    let order = match report.split_first_chunk() {
        Some(([l, r], _)) => l.cmp(r),
        None => return true,
    };
    report
        .iter()
        .tuple_windows()
        .all(|(l, r)| l.cmp(r) == order && matches!(l.abs_diff(*r), 1..=3))
}

fn is_safe_2(mut report: Vec<u32>) -> bool {
    if report.len() < 3 {
        return true;
    }
    let last = report.len() - 1;
    for i in (0..last).rev() {
        if is_safe(&report[..last]) {
            return true;
        }
        report.swap(i, last);
    }
    is_safe(&report[..last])
}

pub fn solve(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|report| is_safe(&report))
        .filter(|safe| *safe)
        .count()
}

pub fn solve_2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(is_safe_2)
        .filter(|safe| *safe)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            solve(
                "7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9"
            ),
            2
        )
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            solve_2(
                "7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9"
            ),
            4
        )
    }
}
