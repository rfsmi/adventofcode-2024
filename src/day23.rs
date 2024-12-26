use std::collections::HashSet;

use itertools::Itertools;

fn parse(input: &str) -> (HashSet<&str>, HashSet<(&str, &str)>) {
    let mut edges = HashSet::new();
    for line in input.trim().lines() {
        let (a, b) = line.trim().split("-").next_tuple().unwrap();
        edges.insert((a, b));
        edges.insert((b, a));
    }
    let names = edges.iter().flat_map(|&(a, b)| [a, b]).collect();
    (names, edges)
}

pub fn solve(input: &str) -> usize {
    let (names, edges) = parse(input);
    names
        .iter()
        .tuple_combinations()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .filter(|(a, b, c)| {
            edges.contains(&(a, b)) && edges.contains(&(a, c)) && edges.contains(&(b, c))
        })
        .count()
}

pub fn solve_2(input: &str) -> String {
    let (names, edges) = parse(input);
    let names = names.into_iter().collect_vec();
    let mut stack = vec![(Vec::<&str>::new(), 0)];
    let mut best = Vec::new();
    while let Some((v, i)) = stack.pop() {
        if i >= names.len() {
            if v.len() > best.len() {
                best = v;
            }
            continue;
        }
        let a = names[i];
        if v.iter().all(|b| edges.contains(&(a, b))) {
            let mut v = v.clone();
            v.push(a);
            stack.push((v, i + 1));
        }
        stack.push((v, i + 1));
    }
    best.into_iter().sorted().join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 7);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), "co,de,ka,ta");
    }
}
