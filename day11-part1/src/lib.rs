fn split_number(num: u64) -> (u64, u64) {
    let halfway_pow_10 = 10_u64.pow(get_digits(num) / 2);

    (num / halfway_pow_10, num % halfway_pow_10)
}

fn get_digits(digit_count: u64) -> u32 {
    digit_count.checked_ilog10().unwrap_or(0) + 1
}

pub fn do_aoc(input: &str) -> usize {
    let mut current: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("non-number?"))
        .collect();
    let mut next = vec![];

    // println!("init: {current:?}");

    for _ in 0..25 {
        for &ele in &current {
            if ele == 0 {
                next.push(1);
            } else if get_digits(ele) % 2 == 0 {
                let (a, b) = split_number(ele);
                next.push(a);
                next.push(b);
            } else {
                next.push(ele * 2024);
            }
        }

        std::mem::swap(&mut current, &mut next);
        // println!("{i}: {current:?}");
        next.clear();
    }

    current.len()
}
