fn str_to_u64(num_str: &str) -> u64 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x | x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as u64))
}

pub fn do_aoc(input: &str) -> u64 {
    let mut numbers = vec![];
    input.lines().filter_map(|line| {
        numbers.clear();
        let (test_value, numbers_str) = line.split_once(": ").unwrap();
        
        let test_value = str_to_u64(test_value);
        numbers.extend(numbers_str.split(' ').map(str_to_u64));

        let operators = numbers.len()-1;

        let first = numbers[0];
        (0..(1<<operators)).any(|case| {
            numbers[1..].iter().enumerate().fold(first, |acc, (i, &number)| {
                let mult = (case&(1<<i))!=0;

                if mult {
                    acc * number
                } else {
                    acc + number
                }
            }) == test_value
        }).then(|| {test_value})
    }).sum()
}