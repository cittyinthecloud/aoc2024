pub fn count_score(board: &[&[u8]], x: usize, y: usize) -> u32 {
    let current_height = board[y][x];

    if current_height == b'9' {
        return 1;
    }

    let mut score = 0;

    // Left
    if x > 0 && board[y][x - 1] == current_height + 1 {
        score += count_score(board, x - 1, y)
    }

    // Right
    if x < board[0].len() - 1 && board[y][x + 1] == current_height + 1 {
        score += count_score(board, x + 1, y)
    }

    // Up
    if y > 0 && board[y - 1][x] == current_height + 1 {
        score += count_score(board, x, y - 1)
    }

    // Down
    if y < board.len() - 1 && board[y + 1][x] == current_height + 1 {
        score += count_score(board, x, y + 1)
    }

    score
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
