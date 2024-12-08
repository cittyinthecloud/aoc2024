use regex::Regex;

fn str_to_i32(num_str: &str) -> i32 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x | x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as i32))
}

pub fn do_aoc(input: &str) -> i32 {

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).map(|m| {
        m.extract::<2>().1.iter().map(|n| str_to_i32(n)).fold(1, |acc, n| acc * n)
    }).sum()
}
