#![feature(iter_map_windows)]

fn str_to_i32(num_str: &str) -> i32 {
    // println!("{num_str}");
    num_str
        .bytes()
        .map(|x| x - b'0')
        .fold(0, |acc, d| (acc * 10) + (d as i32))
}

// fn _is_safe(ascending: bool, have_skipped: bool, current: i32, remaining: &[i32]) -> bool {
//     if remaining.len() == 0 {
//         // got through all the cases
//         return true;
//     }

//     let diff = remaining[0] - current;

//     let sign = if ascending {
//         1
//     } else {
//         -1
//     };

//     if diff.abs() <= 3 && diff.signum() == sign {
//         return _is_safe(ascending, have_skipped, remaining[0], &remaining[1..])
//     } else if !have_skipped {
//         match remaining.len() {
//             0 => unreachable!(), // we would have already returned
//             1 => {return true}, // we remove the remaining one
//             2.. => {
//                 let diff = remaining[1] - current;
//                 let sign = if ascending {1} else {-1};
//                 if diff.abs() <= 3 && diff.signum() == sign {
//                     let res = _is_safe(ascending, true, remaining[1], &remaining[2..]);

//                     if !res{
//                         eprintln!("bad case");
//                         dbg!(ascending);
//                         dbg!(remaining[1]);
//                         dbg!(&remaining[2..]);
//                         eprintln!();
//                     }

//                     return res;
//                 } else {
//                     return false;
//                 }
//             }
//         }
//     } else {
//         return false;
//     }
// }

fn diffs_safe(diffs: &mut impl Iterator<Item = i32>) -> bool {
    let first = diffs.next().expect("lines have at least 1 pair of numbers");
    let first_signum = first.signum();
    matches!(first, 1 | 2 | 3 | -1 | -2 | -3)
        && diffs.all(|x| matches!(x, 1 | 2 | 3 | -1 | -2 | -3) && x.signum() == first_signum)
}

fn is_safe(numbers: &[i32]) -> bool {
    let mut base_iter = numbers.windows(2).map(|x| x[0] - x[1]);

    if diffs_safe(&mut base_iter) {
        return true;
    }

    for removed in 0..numbers.len() {
        let (left_side, right_side) = numbers.split_at(removed);
        let right_side = &right_side[1..];

        // println!("{left_side:?}|{}|{right_side:?}",numbers[removed]);

        let mut the_iter = left_side
            .iter()
            .chain(right_side.iter())
            .map_windows(|[x, y]| *x - *y);
        if diffs_safe(&mut the_iter) {
            // println!("good");
            return true;
        }
    }
    // println!("bad");

    return false;
    // todo!();
}

pub fn do_aoc(input: String) -> usize {
    let mut buffer = vec![];
    input
        .lines()
        .filter(|line| {
            buffer.clear();
            buffer.extend(line.split(" ").map(str_to_i32));
            is_safe(&buffer)
        })
        .count()
}
