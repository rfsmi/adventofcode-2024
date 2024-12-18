use std::{
    collections::{HashSet, VecDeque},
    ops::RangeBounds,
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse(input: &str) -> Vec<(i32, i32)> {
    fn num(input: &str) -> IResult<&str, i32> {
        preceded(multispace0, map_res(digit1, str::parse))(input)
    }
    many1(separated_pair(num, tag(","), num))(input).unwrap().1
}

fn path(
    x_range: impl RangeBounds<i32>,
    y_range: impl RangeBounds<i32>,
    bytes: &[(i32, i32)],
) -> Option<usize> {
    let map: HashSet<_> = bytes.into_iter().collect();
    let mut queue: VecDeque<_> = [(0, (0, 0))].into();
    let mut seen = HashSet::new();
    while let Some((steps, (x, y))) = queue.pop_front() {
        if !x_range.contains(&x)
            || !y_range.contains(&y)
            || map.contains(&(x, y))
            || !seen.insert((x, y))
        {
            continue;
        }
        if !x_range.contains(&(x + 1)) && !y_range.contains(&(y + 1)) {
            return Some(steps);
        }
        for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            queue.push_back((steps + 1, (x + dx, y + dy)));
        }
    }
    None
}

fn compute_2(
    x_range: impl RangeBounds<i32> + Clone,
    y_range: impl RangeBounds<i32> + Clone,
    input: &str,
) -> String {
    let bytes = parse(input);
    let (mut l, mut r) = (0, bytes.len());
    while l < r {
        let mid = l + (r - l) / 2;
        if path(x_range.clone(), y_range.clone(), &bytes[..=mid]).is_some() {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    format!("{},{}", bytes[l].0, bytes[l].1)
}

pub fn solve(input: &str) -> usize {
    path(0..=70, 0..=70, &parse(input)[..1024]).unwrap()
}

pub fn solve_2(input: &str) -> String {
    compute_2(0..=70, 0..=70, input)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    ";

    #[test]
    fn test_sample() {
        assert_eq!(path(0..=6, 0..=6, &parse(SAMPLE)[..12]), Some(22));
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(compute_2(0..=6, 0..=6, SAMPLE), "6,1");
    }
}
