pub fn do_aoc(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut count = 0;

    for x in 0..(width - 2) {
        for y in 0..(height - 2) {
            let rows = &lines[y..=y + 2];

            if rows[1][x + 1] != b'A' {
                continue;
            }

            let mas_ul_dr = (rows[0][x] == b'M' && rows[2][x + 2] == b'S')
                || (rows[0][x] == b'S' && rows[2][x + 2] == b'M');
            let mas_ur_dl = (rows[2][x] == b'M' && rows[0][x + 2] == b'S')
                || (rows[2][x] == b'S' && rows[0][x + 2] == b'M');

            count += (mas_ul_dr && mas_ur_dl) as usize;
        }
    }

    count
}
