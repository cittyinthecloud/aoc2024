pub fn do_aoc(input: &str) -> usize {

    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut count = 0;

    for x in 0..width {
        for y in 0..height {
            if lines[y][x] == b'X' {
                let three_down =  y + 3 < height;
                let three_up = y >= 3;
                let three_left = x >= 3;
                let three_right = x + 3 < width;

                count += 
                (three_down && (lines[y+1][x], lines[y+2][x], lines[y+3][x]) == (b'M', b'A', b'S')) as usize + //Down 
                (three_up && (lines[y-1][x], lines[y-2][x], lines[y-3][x]) == (b'M', b'A', b'S')) as usize + // Up
                (three_left && (lines[y][x-1], lines[y][x-2], lines[y][x-3]) == (b'M', b'A', b'S')) as usize + // Left
                (three_right && (lines[y][x+1], lines[y][x+2], lines[y][x+3]) == (b'M', b'A', b'S')) as usize + // Right
                (three_up && three_left && (lines[y-1][x-1], lines[y-2][x-2], lines[y-3][x-3]) == (b'M', b'A', b'S')) as usize + // Up-Left
                (three_up && three_right && (lines[y-1][x+1], lines[y-2][x+2], lines[y-3][x+3]) == (b'M', b'A', b'S')) as usize + // Up-Right
                (three_down && three_left && (lines[y+1][x-1], lines[y+2][x-2], lines[y+3][x-3]) == (b'M', b'A', b'S')) as usize + // Down-Left
                (three_down && three_right && (lines[y+1][x+1], lines[y+2][x+2], lines[y+3][x+3]) == (b'M', b'A', b'S')) as usize; // Down-Right
            }                                          
        }
    }
    
    count
}