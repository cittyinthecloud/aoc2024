pub fn _count_score(board: &[&[u8]], x: usize, y: usize, found_ends: &mut Vec<(usize, usize)>) {
    let current_height = board[y][x];

    if current_height == b'9' {
        found_ends.push((x, y));
    }

    // let mut score = 0;

    // Left
    if x > 0 && board[y][x - 1] == current_height + 1 {
        _count_score(board, x - 1, y, found_ends)
    }

    // Right
    if x < board[0].len() - 1 && board[y][x + 1] == current_height + 1 {
        _count_score(board, x + 1, y, found_ends)
    }

    // Up
    if y > 0 && board[y - 1][x] == current_height + 1 {
        _count_score(board, x, y - 1, found_ends)
    }

    // Down
    if y < board.len() - 1 && board[y + 1][x] == current_height + 1 {
        _count_score(board, x, y + 1, found_ends)
    }
}

fn count_score(board: &[&[u8]], x: usize, y: usize) -> u32 {
    let mut found_ends = vec![];
    _count_score(board, x, y, &mut found_ends);
    found_ends.sort();
    found_ends.dedup();
    found_ends.len() as u32
}

pub fn do_aoc(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    lines
        .iter()
        .enumerate()
        .map(|(y, &line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == b'0')
                .map(|(x, _)| count_score(&lines, x, y))
                .sum::<u32>()
        })
        .sum()
}
