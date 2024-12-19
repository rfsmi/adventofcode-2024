fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines().map(str::trim).filter(|l| !l.is_empty());
    (lines.next().unwrap().split(", ").collect(), lines.collect())
}

fn count(patterns: &[&str], design: &str) -> usize {
    // memo[i] counts the ways to build the design up to position i
    let mut memo = vec![0; design.len() + 1];
    memo[0] = 1;
    for i in 0..design.len() {
        for pat in patterns {
            if design[i..].starts_with(pat) {
                memo[i + pat.len()] += memo[i];
            }
        }
    }
    memo[design.len()]
}

pub fn solve(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    designs.iter().filter(|d| count(&patterns, d) > 0).count()
}

pub fn solve_2(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    designs.iter().map(|d| count(&patterns, d)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 6);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 16);
    }
}
