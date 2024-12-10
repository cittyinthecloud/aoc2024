use std::{collections::HashMap, hash::Hash, ops::{Add, Sub}};

struct Multimap<K, V> {
        backing: HashMap<K, Vec<V>>,
}

impl<K: Hash + Eq, V> Multimap<K, V> {
    fn new() -> Self {
        Self {
            backing: HashMap::new(),
        }
    }    

    fn insert(&mut self, key: K, val: V) {
        self.backing.entry(key).or_default().push(val);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i32, i32);

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0-rhs.0, self.1-rhs.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0+rhs.0, self.1+rhs.1)
    }
}

pub fn do_aoc(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut antinodes: Vec<Position> = vec![];

    let mut antennas: Multimap<u8, Position> = Multimap::new();
    
    lines.iter().enumerate().flat_map(|(y, row)| {
        row.bytes().enumerate().filter_map(move |(x, cell)| {
            if cell != b'.' {
                Some((cell, Position(x as i32, y as i32)))
            } else {
                None
            }
        })
    }).for_each(|(cell, position)| {
        // println!("{cell}: {position:?}");
        antennas.insert(cell, position);
    });

    for (_, positions) in antennas.backing {
        for (i, &antenna_1) in positions.iter().enumerate() {
            for &antenna_2 in &positions[i+1..] {
                // println!("{antenna_1:?} {antenna_2:?}");
                let offset = antenna_1 - antenna_2;

                let mut cursor: Position = antenna_1;

                // Sub
                while !(cursor.0.is_negative() || cursor.1.is_negative()) && cursor.0 < width && cursor.1 < height {
                    // println!("Sub- {cursor:?} {offset:?}");
                    antinodes.push(cursor);

                    cursor = cursor - offset;
                }

                let mut cursor: Position = antenna_1 + offset;

                // Add
                while !(cursor.0.is_negative() || cursor.1.is_negative()) && cursor.0 < width && cursor.1 < height {
                    // println!("add- {cursor:?} {offset:?}");

                    antinodes.push(cursor);

                    cursor = cursor + offset;
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();
    antinodes.len()
}