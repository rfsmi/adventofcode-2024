use std::{
    cmp::Ordering::{Equal, Greater, Less},
    iter::zip,
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::{map_res, opt, recognize},
    multi::many1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    fn num(input: &str) -> IResult<&str, i64> {
        map_res(recognize(pair(opt(tag("-")), digit1)), str::parse)(input)
    }
    let pos = preceded(tag("p="), separated_pair(num, tag(","), num));
    let vel = preceded(tag("v="), separated_pair(num, tag(","), num));
    let line = preceded(multispace0, separated_pair(pos, space0, vel));
    many1(line)(input).unwrap().1.into_iter().unzip()
}

fn tick(size: (i64, i64), n: i64, pos: &mut [(i64, i64)], vel: &[(i64, i64)]) {
    for (p, v) in zip(pos, vel) {
        p.0 = (p.0 + v.0 * n).rem_euclid(size.0);
        p.1 = (p.1 + v.1 * n).rem_euclid(size.1);
    }
}

fn count(size: (i64, i64), pos: &[(i64, i64)]) -> i64 {
    let mut quads = [[0; 2]; 2];
    for p in pos {
        let x = match (p.0 * 2).cmp(&(size.0 - 1)) {
            Less => 0,
            Greater => 1,
            Equal => continue,
        };
        let y = match (p.1 * 2).cmp(&(size.1 - 1)) {
            Less => 0,
            Greater => 1,
            Equal => continue,
        };
        quads[x][y] += 1;
    }
    quads.into_iter().flatten().product()
}

pub fn solve(input: &str) -> i64 {
    let (mut pos, vel) = parse(input);
    tick((101, 103), 100, &mut pos, &vel);
    count((101, 103), &pos)
}

fn print_robots(size: (i64, i64), pos: &[(i64, i64)]) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            if pos.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn score(size: (i64, i64), pos: &[(i64, i64)]) -> i64 {
    pos.iter()
        .map(|p| {
            let a = (p.0 - size.0 / 2).pow(2) as f64;
            let b = (p.1 - size.1 / 2).pow(2) as f64;
            (a + b).sqrt()
        })
        .sum::<f64>() as i64
}

pub fn solve_2(input: &str) -> i64 {
    let (pos, vel) = parse(input);
    let (_, n, pos) = (0..10000)
        .map(|n| {
            let mut pos = pos.clone();
            tick((101, 103), n, &mut pos, &vel);
            (score((101, 103), &pos), n, pos)
        })
        .min()
        .unwrap();
    print_robots((101, 103), &pos);
    n
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    ";

    #[test]
    fn test_sample() {
        let (mut pos, vel) = parse(SAMPLE);
        tick((11, 7), 100, &mut pos, &vel);
        assert_eq!(count((11, 7), &pos), 12)
    }
}
