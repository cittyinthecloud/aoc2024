use std::hint::unreachable_unchecked;

fn str_to_u64(num_str: &str) -> u64 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x | x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as u64))
}

fn dec_shift_helper(num: u64) -> u64 {
    match num {
        0..10 => 10,
        10..100 => 100,
        100..1000 => 1000,
        1000..10000 => 10000,
        // Safety: it isn't :)
        _ => { unsafe { unreachable_unchecked() } }
    }
}

fn _test_value_internal(test_value: u64, acc: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return test_value == acc
    } else {
        _test_value_internal(test_value, acc + numbers[0], &numbers[1..])||
        _test_value_internal(test_value, acc * numbers[0], &numbers[1..])||
        _test_value_internal(test_value, acc * dec_shift_helper(numbers[0]) + numbers[0], &numbers[1..])
    }
}

fn check_test_value(test_value: u64, numbers: &[u64]) -> bool {
    _test_value_internal(test_value, numbers[0], &numbers[1..])
}

pub fn do_aoc(input: &str) -> u64 {
    let mut numbers = vec![];
    input.lines().filter_map(|line| {
        numbers.clear();
        let (test_value, numbers_str) = line.split_once(": ").unwrap();
        
        let test_value = str_to_u64(test_value);
        numbers.extend(numbers_str.split(' ').map(str_to_u64));

        check_test_value(test_value, &numbers).then(|| {test_value})
    }).sum()
}