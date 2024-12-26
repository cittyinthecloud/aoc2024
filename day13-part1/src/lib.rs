fn str_to_u32(num_str: &str) -> u32 {
    num_str
        .bytes()
        .map(|x| x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as u32))
}

pub fn do_aoc(input: &str) -> u32 {
    input
        .split("\n\n")
        .filter_map(|x| {
            let mut lines = x.lines();

            let line_1 = lines.next().expect("case has 3 lines");
            let line_2 = lines.next().expect("case has 3 lines");
            let line_3 = lines.next().expect("case has 3 lines");

            let a_x = str_to_u32(&line_1[12..14]);
            let a_y = str_to_u32(&line_1[18..20]);
            let b_x = str_to_u32(&line_2[12..14]);
            let b_y = str_to_u32(&line_2[18..20]);

            let (_, rest) = line_3.split_once('=').expect("third line format");
            let target_x = rest
                .bytes()
                .map_while(|x| {
                    if x.is_ascii_digit() {
                        Some(x - b'0')
                    } else {
                        None
                    }
                })
                .fold(0, |acc: u32, d| (acc * 10) + (d as u32));
            let (_, rest) = rest.split_once('=').expect("third line format");
            let target_y = rest
                .bytes()
                .map_while(|x| {
                    if x.is_ascii_digit() {
                        Some(x - b'0')
                    } else {
                        None
                    }
                })
                .fold(0, |acc: u32, d| (acc * 10) + (d as u32));

            (0..=100)
                .flat_map(|a| std::iter::repeat(a).zip(0..=100))
                .filter(|(a, b)| {
                    target_x == a * a_x + b * b_x && target_y == a * a_y + b * b_y
                }).map(|(a,b)| 3*a + b)
                .min()
        })
        .sum()
}
