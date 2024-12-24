use std::fs;

use day12_part2::do_aoc;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let res = do_aoc(&file);
    println!("{res}")
}
