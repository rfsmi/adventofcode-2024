use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
}

#[derive(Default)]
struct Maze {
    start: State,
    end: (i32, i32),
    walls: HashSet<(i32, i32)>,
}

fn parse(input: &str) -> Maze {
    let mut maze = Maze::default();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let pos = (y as i32, x as i32);
            match c {
                'S' => maze.start = State { pos, dir: (0, 1) },
                'E' => maze.end = pos,
                '#' => {
                    maze.walls.insert(pos);
                }
                _ => (),
            }
        }
    }
    maze
}

fn dijkstra(
    walls: &HashSet<(i32, i32)>,
    starts: impl IntoIterator<Item = State>,
) -> HashMap<State, usize> {
    let mut queue: BinaryHeap<_> = starts.into_iter().map(|s| Reverse((0, s))).collect();
    let mut distances = HashMap::new();
    while let Some(Reverse((score, state))) = queue.pop() {
        if walls.contains(&state.pos) || distances.contains_key(&state) {
            continue;
        }
        distances.insert(state, score);
        let State {
            pos: (y, x),
            dir: (dy, dx),
        } = state;
        for (score, pos, dir) in [
            (score + 1, (y + dy, x + dx), state.dir),
            (score + 1000, state.pos, (dx, -dy)),
            (score + 1000, state.pos, (-dx, dy)),
        ] {
            queue.push(Reverse((score, State { pos, dir })));
        }
    }
    distances
}

pub fn solve(input: &str) -> usize {
    let maze = parse(input);
    dijkstra(&maze.walls, [maze.start])
        .into_iter()
        .filter_map(|(state, score)| (state.pos == maze.end).then_some(score))
        .min()
        .unwrap()
}

pub fn solve_2(input: &str) -> usize {
    let best_score = solve(input);
    let maze = parse(input);
    let ends = [(-1, 0), (1, 0), (0, -1), (0, 1)].map(|dir| State { pos: maze.end, dir });
    let from_end = dijkstra(&maze.walls, ends);
    dijkstra(&maze.walls, [maze.start])
        .into_iter()
        .filter_map(|(mut state, score_1)| {
            // State -> End == End -> reversed(State)
            state.dir = (-state.dir.0, -state.dir.1);
            let score_2 = from_end.get(&state)?;
            (score_1 + score_2 == best_score).then_some(state.pos)
        })
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 7036);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 45);
    }
}
