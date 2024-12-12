use std::{cmp::Reverse, collections::BinaryHeap, mem::take};

struct File {
    len: usize,
    val: usize,
}

struct FS {
    files: Vec<(usize, File)>,
    empty_pos: [BinaryHeap<Reverse<usize>>; 10],
}

impl FS {
    fn new(input: &str) -> Self {
        let mut files = Vec::new();
        let mut empty_pos: [BinaryHeap<Reverse<usize>>; 10] = Default::default();
        let mut pos = 0;
        for (i, c) in input.trim().chars().enumerate() {
            let len = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                files.push((pos, File { len, val: i / 2 }));
            } else {
                empty_pos[len].push(Reverse(pos));
            }
            pos += len;
        }
        Self { files, empty_pos }
    }

    fn split_files(mut self) -> Self {
        for (pos, file) in take(&mut self.files).into_iter() {
            for i in 0..file.len {
                self.files.push((pos + i, File { len: 1, ..file }));
            }
        }
        self
    }

    fn compact(mut self) -> Vec<(usize, File)> {
        for (mut pos, file) in take(&mut self.files).into_iter().rev() {
            let first_space = (file.len..self.empty_pos.len())
                .into_iter()
                .filter(|&i| !self.empty_pos[i].is_empty())
                .filter(|&i| self.empty_pos[i].peek().unwrap().0 < pos)
                .min_by_key(|&i| self.empty_pos[i].peek().unwrap().0);
            if let Some(space) = first_space {
                pos = self.empty_pos[space].pop().unwrap().0;
                if space > file.len {
                    self.empty_pos[space - file.len].push(Reverse(pos + file.len));
                }
            };
            self.files.push((pos, file))
        }
        self.files.sort_by_key(|&(pos, _)| pos);
        self.files
    }
}

fn checksum(files: impl IntoIterator<Item = (usize, File)>) -> usize {
    files
        .into_iter()
        .map(|(pos, File { len, val })| val * len * (2 * pos + len - 1) / 2)
        .sum()
}

pub fn solve(input: &str) -> usize {
    checksum(FS::new(input).split_files().compact())
}

pub fn solve_2(input: &str) -> usize {
    checksum(FS::new(input).compact())
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
