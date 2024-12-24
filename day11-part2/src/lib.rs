use std::collections::HashMap;

fn get_digits(digit_count: u64) -> u32 {
    digit_count.checked_ilog10().unwrap_or(0) + 1
}

fn _blink_uncached(stone: u64, count: u32, cache: &mut HashMap<(u64,u32),u64>) -> u64 {
    // println!("{} {stone}", " ".repeat(count as usize));
    if count == 75 {
        1
    } else if stone == 0 {
        _blink(1, count + 1, cache)
    } else {
        let stone_digits = get_digits(stone);
        if  stone_digits % 2 == 0 {
            let halfway_pow_10 = 10u64.pow(stone_digits / 2);
            _blink(stone / halfway_pow_10, count + 1, cache) + _blink(stone % halfway_pow_10, count + 1, cache)
        } else {
            _blink(stone * 2024, count + 1, cache)
        }
    } 
}

fn _blink(stone: u64, count: u32, cache: &mut HashMap<(u64,u32),u64>) -> u64 {
    match cache.get(&(stone, count)) {
        Some(res) => {
            *res
        },
        None => {
            let res = _blink_uncached(stone, count, cache);
            cache.insert((stone, count), res);
            res
        },
    }
}

fn blink(stone: u64, cache: &mut HashMap<(u64,u32),u64>) -> u64 {
    _blink(stone, 0, cache)
}

pub fn do_aoc(input: &str) -> u64 {
    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    input
        .trim()
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().expect("non-number?"))
        .map(|stone| blink(stone, &mut cache))
        .sum()
}
