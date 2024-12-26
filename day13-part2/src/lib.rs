fn str_to_u64(num_str: &str) -> u64 {
    num_str
        .bytes()
        .map(|x| x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as u64))
}

pub fn do_aoc(input: &str) -> i64 {
    input
        .split("\n\n")
        .filter_map(|x| {
            let mut lines = x.lines();

            let line_1 = lines.next().expect("case has 3 lines");
            let line_2 = lines.next().expect("case has 3 lines");
            let line_3 = lines.next().expect("case has 3 lines");

            let a_x = str_to_u64(&line_1[12..14]);
            let a_y = str_to_u64(&line_1[18..20]);
            let b_x = str_to_u64(&line_2[12..14]);
            let b_y = str_to_u64(&line_2[18..20]);

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
                .fold(0, |acc, d| (acc * 10) + (d as u64)) + 10000000000000;
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
                .fold(0, |acc, d| (acc * 10) + (d as u64)) + 10000000000000;
            
            let det: i64 = (a_x * b_y) as i64 - (b_x * a_y) as i64;
            let det_x: i64 = (target_x * b_y) as i64 - (b_x * target_y) as i64;
            let det_y: i64 = (a_x * target_y) as i64 - (a_y * target_x) as i64;

            if !(det == 0 || det_x % det != 0 || det_y % det != 0) && det_x.is_negative() == det_y.is_negative() && det_y.is_negative() == det.is_negative() {
                Some(3 * (det_x/det) + (det_y/det))
            } else {
                None
            }
        }).sum()
}
