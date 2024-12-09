use itertools::Itertools;

fn parse(input: &str) -> Vec<(u64, u64, u64)> {
    let mut nums = Vec::new();
    for c in input.trim().chars() {
        nums.push(c.to_digit(10).unwrap() as u64);
    }
    if nums.len() % 2 != 0 {
        nums.push(0);
    }
    let mut counts = Vec::new();
    for (id, (num, trailing_space)) in nums.into_iter().tuples().enumerate() {
        counts.push((num, id as u64, trailing_space));
    }
    counts
}

fn checksum(counts: &[(u64, u64, u64)]) -> u64 {
    let (mut sum, mut pos) = (0, 0);
    for (n, id, space) in counts {
        sum += id * n * pos;
        sum += id * n * (n - 1) / 2;
        pos += n + space;
    }
    sum
}

pub fn solve(input: &str) -> u64 {
    let mut counts = parse(input);
    let mut i = 0;
    while i < counts.len() {
        let (n, id, space) = counts[i];
        if space == 0 {
            i += 1;
            continue;
        }
        let j = counts.len() - 1;
        if i >= j {
            break;
        }
        let (n1, id1, _) = counts[j];
        if n1 <= space {
            counts[i] = (n, id, 0);
            counts.remove(j);
            counts.insert(i + 1, (n1, id1, space - n1));
        } else {
            counts[i] = (n, id, 0);
            counts[j] = (n1 - space, id1, 0);
            counts.insert(i + 1, (space, id1, 0));
        }
    }
    checksum(&counts)
}

pub fn solve_2(input: &str) -> u64 {
    let mut counts = parse(input);
    let mut min_id = u64::MAX;
    let mut j = counts.len() - 1;
    while j > 0 {
        let (n, id, space) = counts[j];
        if id >= min_id {
            j -= 1;
            continue;
        }
        min_id = id;
        let Some(i) = (0..j).find(|&i| counts[i].2 >= n) else {
            j -= 1;
            continue;
        };
        let (n1, id1, space1) = counts[i];
        counts[i] = (n1, id1, 0);
        counts.remove(j);
        counts.insert(i + 1, (n, id, space1 - n));
        counts[j].2 += n + space;
    }
    checksum(&counts)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(solve("2333133121414131402"), 1928)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2("2333133121414131402"), 2858)
    }
}
