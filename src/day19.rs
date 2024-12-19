fn parse(input: &str) -> (Vec<&[u8]>, Vec<&[u8]>) {
    let mut lines = input.lines().map(str::trim).filter(|l| !l.is_empty());
    let first_line = lines.next().unwrap();
    let patterns = first_line.split(",").map(|p| p.trim().as_bytes()).collect();
    (patterns, lines.map(str::as_bytes).collect())
}

fn arrangements(patterns: &[&[u8]], design: &[u8]) -> usize {
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
    designs
        .into_iter()
        .filter(|d| arrangements(&patterns, d) > 0)
        .count()
}

pub fn solve_2(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    designs
        .into_iter()
        .map(|d| arrangements(&patterns, d))
        .sum()
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
