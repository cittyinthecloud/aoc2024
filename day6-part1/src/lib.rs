
use memchr::memchr;

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn shift_position(&self, cur_pos: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::UP => (cur_pos.0, cur_pos.1 - 1),
            Direction::DOWN => (cur_pos.0, cur_pos.1 + 1),
            Direction::LEFT => (cur_pos.0 - 1, cur_pos.1),
            Direction::RIGHT => (cur_pos.0 + 1, cur_pos.1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

struct Board<'a> {
    lines: Vec<&'a [u8]>,
    width: usize,
    height: usize,
}

impl<'a> Board<'a> {
    fn new(lines: Vec<&'a [u8]>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        Self {
            lines,
            width,
            height,
        }
    }
}

impl<'a> Board<'a> {
    fn get(&self, index: (i32, i32)) -> Option<u8> {
        let x_index: Option<usize> = index.0.try_into().ok();
        let y_index: Option<usize> = index.1.try_into().ok();

        if let Some(x_index) = x_index {
            if let Some(y_index) = y_index {
                if x_index >= self.width || y_index >= self.height {
                    return None;
                } else {
                    return Some(self.lines[y_index][x_index]);
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
struct VecSet<T> {
    backing: Vec<T>,
}

impl<T: PartialEq> VecSet<T> {
    fn new() -> Self {
        Self { backing: vec![] }
    }

    fn len(&self) -> usize {
        self.backing.len()
    }

    fn insert(&mut self, elem: T) {
        if !self.backing.contains(&elem) {
            self.backing.push(elem);
        }
    }
}

pub fn do_aoc(input: &str) -> usize {
    let mut start_pos = None;

    // This is gross but it does it in 1 pass.
    let board = Board::new(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let line = line.as_bytes();

                if let Some(x) = memchr(b'^', line) {
                    start_pos = Some((x as i32, y as i32));
                }

                line
            })
            .collect(),
    );

    let mut position = start_pos.unwrap();
    let mut direction = Direction::UP;

    // let mut visited = HashSet::new();
    // visited.insert(position);

    let mut visited = vec![position];

    loop {
        let forward_pos = direction.shift_position(position);

        if let Some(chr) = board.get(forward_pos) {
            if chr == b'#' {
                // There's a wall in front of us, turn.
                direction = direction.turn_right();
            } else {
                // It's an empty cell, we can move into it.
                position = forward_pos;
                visited.push(position);
            }
        } else {
            // We've left the board
            break;
        }
    }

    visited.sort();
    visited.dedup();
    visited.len()
}
