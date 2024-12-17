use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Map {
    walls: HashSet<(i32, i32)>,
    boxes: HashMap<(i32, i32), usize>,
    box_positions: Vec<Vec<(i32, i32)>>,
    robot: (i32, i32),
}

impl Map {
    fn push(&self, box_id: usize, (dy, dx): (i32, i32)) -> Option<HashSet<usize>> {
        let mut ids = HashSet::new();
        let mut stack = Vec::new();
        for &pos in &self.box_positions[box_id] {
            stack.push((box_id, pos));
        }
        while let Some((id, (y, x))) = stack.pop() {
            let (y, x) = (y + dy, x + dx);
            if self.walls.contains(&(y, x)) {
                return None;
            }
            ids.insert(id);
            if let Some(&next_id) = self.boxes.get(&(y, x)) {
                if next_id != id {
                    for &pos in &self.box_positions[next_id] {
                        stack.push((next_id, pos));
                    }
                }
            };
        }
        Some(ids)
    }

    fn step(&mut self, (dy, dx): (i32, i32)) {
        let (y, x) = (self.robot.0 + dy, self.robot.1 + dx);
        if self.walls.contains(&(y, x)) {
            return;
        }
        if let Some(&box_id) = self.boxes.get(&(y, x)) {
            let Some(move_ids) = self.push(box_id, (dy, dx)) else {
                return;
            };
            // Remove boxes from their old positions
            for &id in &move_ids {
                for (y, x) in &mut self.box_positions[id] {
                    self.boxes.remove(&(*y, *x));
                    (*y, *x) = (*y + dy, *x + dx);
                }
            }
            // Insert boxes at their new positions
            for id in move_ids {
                for &pos in &self.box_positions[id] {
                    self.boxes.insert(pos, id);
                }
            }
        }
        self.robot = (y, x);
    }

    fn gps(&self) -> i64 {
        self.box_positions
            .iter()
            .flat_map(|positions| positions.iter().min())
            .map(|&(y, x)| (100 * y + x) as i64)
            .sum()
    }
}

fn compute<const P2: bool>(input: &str) -> i64 {
    let mut map = Map::default();
    let mut lines = input.trim().lines().enumerate();
    while let Some((y, line)) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        for (x, c) in line.trim().chars().enumerate() {
            let positions = if P2 {
                vec![(y as i32, 2 * x as i32), (y as i32, 1 + 2 * x as i32)]
            } else {
                vec![(y as i32, x as i32)]
            };
            match c {
                '@' => map.robot = positions[0],
                '#' => {
                    map.walls.extend(positions);
                }
                'O' => {
                    for &pos in &positions {
                        map.boxes.insert(pos, map.box_positions.len());
                    }
                    map.box_positions.push(positions);
                }
                _ => (),
            }
        }
    }
    for c in lines.flat_map(|(_, line)| line.trim().chars()) {
        let dir = match c {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => panic!("Unexpected char {c}"),
        };
        map.step(dir);
    }
    map.gps()
}

pub fn solve(input: &str) -> i64 {
    compute::<false>(input)
}

pub fn solve_2(input: &str) -> i64 {
    compute::<true>(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 10092);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 9021);
    }
}
