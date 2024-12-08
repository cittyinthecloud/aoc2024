#![feature(iter_map_windows)]

fn str_to_i32(num_str: &str) -> i32 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x| x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as i32))
}

pub fn do_aoc(input: String) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut iter = line
                .split_ascii_whitespace()
                .map_windows(|[x, y]| str_to_i32(x) - str_to_i32(y));
            let first = iter.next().expect("lines have at least 1 pair of numbers");

            let first_signum = first.signum();

            matches!(first, 1|2|3|-1|-2|-3) && iter.all(|x| matches!(x, 1|2|3|-1|-2|-3) && x.signum() == first_signum)
        })
        .count()
}
