use std::iter::zip;

use itertools::iproduct;

fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let (mut keys, mut locks) = (Vec::new(), Vec::new());
    let mut chars = input.chars().filter(|&c| c == '.' || c == '#');
    loop {
        let mut heights = [0; 5];
        let mut is_lock = false;
        for y in 0..7 {
            for x in 0..5 {
                match chars.next() {
                    Some('#') => heights[x] += 1,
                    Some(_) => continue,
                    None => return (keys, locks),
                }
            }
            if y == 0 {
                is_lock = heights.iter().all(|&h| h == 1);
            }
        }
        if is_lock { &mut locks } else { &mut keys }.push(heights);
    }
}

pub fn solve(input: &str) -> usize {
    let (keys, locks) = parse(input);
    iproduct!(keys, locks)
        .filter(|(k, l)| zip(k, l).all(|(h1, h2)| h1 + h2 <= 7))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 3);
    }
}
