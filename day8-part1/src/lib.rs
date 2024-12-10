use std::{collections::HashMap, hash::Hash};

struct Multimap<K, V> {
        backing: HashMap<K, Vec<V>>,
}

impl<K: Hash + Eq, V> Multimap<K, V> {
    fn new() -> Self {
        Self {
            backing: HashMap::new(),
        }
    }

    fn get(&self, key: &K) -> &[V] {
        if let Some(vals) = self.backing.get(key) {
            &vals[..]
        } else {
            &[]
        }
    }

    

    fn insert(&mut self, key: K, val: V) {
        self.backing.entry(key).or_default().push(val);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position(i32, i32);


pub fn do_aoc(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut antinodes = vec![];

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
        for (i, antenna_1) in positions.iter().enumerate() {
            for antenna_2 in &positions[i+1..] {
                // println!("{antenna_1:?} {antenna_2:?}");
                let x_offset = antenna_1.0 - antenna_2.0;
                let y_offset = antenna_1.1 - antenna_2.1;

                let thing_1= Position(antenna_1.0 - x_offset,antenna_1.1 - y_offset);
                let thing_2 = Position(antenna_2.0 - x_offset,antenna_2.1 - y_offset);
                let thing_3 = Position(antenna_1.0 + x_offset,antenna_1.1 + y_offset);
                let thing_4 = Position(antenna_2.0 + x_offset,antenna_2.1 + y_offset);


                for ele in [thing_1, thing_2, thing_3, thing_4] {
                    if ele != *antenna_1 && ele != *antenna_2 && !ele.0.is_negative() && !ele.1.is_negative() && ele.0 < width && ele.1 < height {
                        antinodes.push(ele);
                    }
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();
    antinodes.len()
}