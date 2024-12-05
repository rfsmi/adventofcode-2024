use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn parse(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    fn num(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }
    let rule = separated_pair(num, tag("|"), num);
    let rules = map(many1(preceded(multispace0, rule)), |rules| {
        rules.into_iter().collect()
    });
    let update = separated_list1(tag(","), num);
    let updates = many1(preceded(multispace1, update));
    pair(rules, updates)(input).unwrap().1
}

fn difference<'a, T, Q>(
    a: impl IntoIterator<Item = T>,
    b: impl IntoIterator<Item = Q>,
) -> impl Iterator<Item = T>
where
    T: 'a + Hash + Eq,
    Q: Eq + Hash + Borrow<T>,
{
    let b: HashSet<Q> = b.into_iter().collect();
    a.into_iter().filter(move |n| !b.contains(n))
}

fn topo_sort(rules: &HashSet<(u32, u32)>, nodes: &[u32]) -> Option<Vec<u32>> {
    // edges maps from pre-nodes to after-nodes.
    let mut edges = HashMap::<u32, Vec<u32>>::new();
    // num_deps maps nodes to the number of nodes that have to precede it.
    let mut num_deps = HashMap::<u32, usize>::new();
    let mut all_ok = true;
    for (&a, &b) in nodes.iter().tuple_combinations() {
        if rules.contains(&(a, b)) {
            edges.entry(a).or_default().push(b);
            *num_deps.entry(b).or_default() += 1;
        }
        if rules.contains(&(b, a)) {
            edges.entry(b).or_default().push(a);
            *num_deps.entry(a).or_default() += 1;
            all_ok = false;
        }
    }
    if all_ok {
        return None;
    }
    // next_nodes is the set of nodes that don't depend on any others.
    let mut next_nodes: Vec<u32> = difference(nodes, num_deps.keys()).copied().collect();
    let mut result = Vec::new();
    while let Some(n) = next_nodes.pop() {
        result.push(n);
        for &d in edges.get(&n).into_iter().flatten() {
            let num = num_deps.get_mut(&d).unwrap();
            *num -= 1;
            if *num == 0 {
                next_nodes.push(d);
            }
        }
    }
    Some(result)
}

pub fn solve(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|update| topo_sort(&rules, update).is_none())
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn solve_2(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter_map(|update| topo_sort(&rules, &update))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 143)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 123)
    }
}
