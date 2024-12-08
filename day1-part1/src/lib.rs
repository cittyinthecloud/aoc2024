fn str_to_u32(num_str: &str) -> u32 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x| x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as u32))
}

pub fn do_aoc(input: String) -> u32 {
    let lines = input.lines();
    // let count = lines.clone().count();
    let (mut a, mut b): (Vec<_>, Vec<_>) = lines
        .map(|line| {
            let (a, b) = (&line[0..5], &line[5 + 3..]);
            (str_to_u32(a), str_to_u32(b))
        })
        .unzip();

    a.sort_unstable();
    b.sort_unstable();

    a.iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(*b))
}
