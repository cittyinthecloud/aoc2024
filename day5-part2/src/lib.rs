#![feature(iter_collect_into)]

fn str_to_u32(num_str: &str) -> u32 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x| x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as u32))
}

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

    fn put(&mut self, key: K, val: V) {
        self.backing.entry(key).or_default().push(val);
    }
}

pub fn do_aoc(input: &str) -> u32 {
    let (rules, cases) = input.split_once("\n\n").unwrap();

    let mut map: Multimap<u32, u32> = Multimap::new();
    for line in rules.lines() {
        let before = str_to_u32(&line[0..2]);
        let after = str_to_u32(&line[3..5]);

        map.put(after, before);
    }

    // let mut line = vec![];
    let mut must_not = vec![];

    cases
        .lines()
        .filter_map(|case| {
            let mut line: Vec<_> = case.split(",").map(str_to_u32).collect();
            let mut is_unsorted_line = false;
            loop {
                must_not.clear();

                let mut needs_moved = None;

                for (i, n) in line.iter().enumerate() {
                    if must_not.contains(n) {
                        is_unsorted_line = true;
                        needs_moved = Some(i);
                        break;
                    }

                    must_not.extend(map.get(n));
                }

                if let Some(i) = needs_moved {
                    let remove = line.remove(i);
                    line.insert(0, remove);
                } else {
                    break;
                }
            }

            is_unsorted_line.then(|| line[line.len() / 2])
        })
        .sum()
}
