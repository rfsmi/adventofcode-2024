use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::from_fn,
};

use itertools::{chain, iproduct, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, digit1, multispace0, space0},
    combinator::{map, map_opt, verify},
    multi::many1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn and(a: bool, b: bool) -> bool {
    a & b
}

fn or(a: bool, b: bool) -> bool {
    a | b
}

fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

fn parse(
    input: &str,
) -> (
    Vec<(String, bool)>,
    HashMap<String, (String, String, fn(bool, bool) -> bool)>,
) {
    fn reg(input: &str) -> IResult<&str, String> {
        map(
            preceded(space0, verify(alphanumeric0, |s: &str| s.len() == 3)),
            str::to_string,
        )(input)
    }
    fn bool_num(input: &str) -> IResult<&str, bool> {
        preceded(
            space0,
            map_opt(digit1, |n| match n {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            }),
        )(input)
    }
    let op = preceded(
        space0,
        map_opt(alpha1, |name| match name {
            "AND" => Some(and as fn(bool, bool) -> bool),
            "OR" => Some(or as fn(bool, bool) -> bool),
            "XOR" => Some(xor as fn(bool, bool) -> bool),
            _ => None,
        }),
    );
    let value = separated_pair(reg, tag(":"), bool_num);
    let edge = tuple((reg, op, reg, preceded(tag(" ->"), reg)));
    let (input, init) = many1(preceded(multispace0, value))(input).unwrap();
    let (_, edges) = many1(preceded(multispace0, edge))(input).unwrap();
    let edges = edges
        .into_iter()
        .map(|(a, op, b, c)| (c.clone(), (a.clone(), b.clone(), op)))
        .collect();
    (init, edges)
}

fn read_num(vals: &HashMap<String, bool>, prefix: char) -> u64 {
    vals.into_iter()
        .filter_map(|(a, v)| a.strip_prefix(prefix).map(|a| (a, v)))
        .filter_map(|(n, v)| n.parse::<u8>().ok().map(|n| (n, v)))
        .map(|(n, &v)| (v as u64) << n)
        .sum()
}

fn compute(
    init: impl IntoIterator<Item = (String, bool)>,
    edges: &HashMap<String, (String, String, fn(bool, bool) -> bool)>,
) -> Option<HashMap<String, bool>> {
    let mut todo: Vec<(String, bool)> = init.into_iter().collect_vec();
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    let mut reqs: HashMap<String, usize> = HashMap::new();
    let mut vals: HashMap<String, bool> = HashMap::new();
    for (c, (a, b, _)) in edges {
        deps.entry(a.clone()).or_default().push(c.clone());
        deps.entry(b.clone()).or_default().push(c.clone());
        reqs.insert(c.clone(), if a == b { 1 } else { 2 });
    }
    while let Some((a, v)) = todo.pop() {
        let a_deps = deps.get(&a);
        vals.insert(a, v);
        for c in a_deps.into_iter().flatten() {
            let Some(count) = reqs.get_mut(c) else {
                continue;
            };
            if *count == 0 {
                return None; // There is a loop
            }
            *count -= 1;
            if *count == 0 {
                let (a, b, op) = edges.get(c).unwrap();
                let &a = vals.get(a).unwrap();
                let &b = vals.get(b).unwrap();
                todo.push((c.clone(), op(a, b)));
            }
        }
    }
    if reqs.into_values().any(|v| v != 0) {
        None
    } else {
        Some(vals)
    }
}

pub fn solve(input: &str) -> u64 {
    let (init, edges) = parse(input);
    let vals = compute(init, &edges).unwrap();
    read_num(&vals, 'z')
}

fn swap_map_keys<K: Hash + Eq + Clone, V>(map: &mut HashMap<K, V>, a: &K, b: &K) {
    let a_val = map.remove(a).unwrap();
    let b_val = map.remove(b).unwrap();
    map.insert(a.clone(), b_val);
    map.insert(b.clone(), a_val);
}

const BITS: u32 = 45;

fn make_init(prefix: char, num: u64) -> impl Iterator<Item = (String, bool)> {
    (0..BITS).map(move |i| (format!("{prefix}{i:02}"), (num >> i) & 1 != 0))
}

fn output_deps<'a>(
    edges: &'a HashMap<String, (String, String, fn(bool, bool) -> bool)>,
    start: &'a str,
) -> impl Iterator<Item = &'a str> {
    let mut stack = vec![start];
    from_fn(move || {
        while let Some(o) = stack.pop() {
            if let Some((a, b, _)) = edges.get(o) {
                stack.push(a);
                stack.push(b);
                return Some(o);
            };
        }
        None
    })
}

fn check_adder_n(
    edges: &HashMap<String, (String, String, fn(bool, bool) -> bool)>,
    n: u32,
) -> bool {
    let (shift, len) = match n {
        0 => (0, 2),
        BITS => (n - 1, 2),
        _ => (n - 1, 4),
    };
    for (x, y) in iproduct!(0..len, 0..len) {
        let (x, y) = (x << shift, y << shift);
        let Some(vals) = compute(chain!(make_init('x', x), make_init('y', y)), edges) else {
            return false;
        };
        if vals.get(&format!("z{n:02}")) != Some(&((x + y) & (1 << n) != 0)) {
            return false;
        }
    }
    true
}

fn find_solution(
    edges: &mut HashMap<String, (String, String, fn(bool, bool) -> bool)>,
    good_bits: u32,
    mut swappable_outputs: HashSet<String>,
    swaps: usize,
) -> Option<Vec<String>> {
    let Some(bad_bit) = (good_bits..=BITS).find(|&bit| !check_adder_n(&edges, bit)) else {
        return Some(vec![]);
    };
    if swaps == 4 {
        return None;
    }
    for bit in good_bits..bad_bit {
        for a in output_deps(&edges, &format!("z{bit:02}")) {
            swappable_outputs.remove(a);
        }
    }
    for (a, b) in swappable_outputs.iter().tuple_combinations() {
        swap_map_keys(edges, a, b);
        if check_adder_n(&edges, bad_bit) {
            if let Some(mut solution) =
                find_solution(edges, bad_bit, swappable_outputs.clone(), swaps + 1)
            {
                solution.extend([a.clone(), b.clone()]);
                return Some(solution);
            }
        }
        swap_map_keys(edges, a, b);
    }
    None
}

pub fn solve_2(input: &str) -> String {
    let (_, mut edges) = parse(input);
    println!("check 45 {:?}", check_adder_n(&edges, 45));
    let swappable_outputs: HashSet<_> = edges.keys().cloned().collect();
    let swaps = find_solution(&mut edges, 0, swappable_outputs, 0).unwrap();
    swaps.into_iter().sorted().join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 2024);
    }
}
