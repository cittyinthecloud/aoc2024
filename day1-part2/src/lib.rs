use std::collections::HashMap;
use std::hash::Hash;

fn str_to_u32(num_str: &str) -> u32 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x| x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as u32))
}

// Tried doing this in a type, because funny

struct IterCounter<K>
where
    K: Eq + Hash,
{
    map: HashMap<K, u32>,
}

impl<K: Eq + Hash> IterCounter<K> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: K) -> u32 {
        if let Some(v) = self.map.get(&key) {
            *v
        } else {
            0
        }
    }

    pub fn get_mut(&mut self, key: K) -> &mut u32 {
        self.map.entry(key).or_insert(0)
    }

    pub fn incr(&mut self, key: K) {
        *self.get_mut(key) += 1
    }
}

impl<K: Eq + Hash> Default for IterCounter<K> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Eq + Hash> Extend<K> for IterCounter<K> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for k in iter {
            self.incr(k);
        }
    }
}

pub fn do_aoc(input: String) -> u32 {
    let lines = input.lines();
    // let count = lines.clone().count();
    let (mut a, mut b): (Vec<_>, IterCounter<_>) = lines
        .map(|line| {
            let (a, b) = (&line[0..5], &line[5 + 3..]);
            (str_to_u32(a), str_to_u32(b))
        })
        .unzip();

    a.iter().map(|x| x * b.get(*x)).sum()
}
