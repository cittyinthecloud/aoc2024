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

pub fn do_aoc(input: &str) -> u64 {
    let mut numbers = vec![];
    input.lines().filter_map(|line| {
        numbers.clear();
        let (test_value, numbers_str) = line.split_once(": ").unwrap();
        
        let test_value = str_to_u64(test_value);
        numbers.extend(numbers_str.split(' ').map(str_to_u64));

        let operators = (numbers.len()-1) as u32;

        let first = numbers[0];

        // println!("{test_value}");
        (0..(3u32.pow(operators))).any(|case| {
            // println!(" {case}");
            let mut thing = 1;
            numbers[1..].iter().fold(first, |acc, &number| {
                let x: u32 = (case / thing) % 3;
                // print!("  {x}: {acc} {number} = ");
                let res = match x {
                    0 => acc + number,
                    1 => acc * number,
                    2 => {
                        (acc * dec_shift_helper(number)) + number
                    }
                    _ =>  { unsafe { unreachable_unchecked() } },
                };                
                thing *= 3;
                res  
            }) == test_value
        }).then(|| {test_value})
    }).sum()
}