use bitvec::prelude::*;

struct Board<'input> {
    backing: &'input [u8],
    width: usize,
    height: usize,
}

impl<'input> Board<'input> {
    fn new(input: &'input str) -> Self {
        let height = input.lines().count();
        let width = input
            .lines()
            .next()
            .expect("Input contains at least 1 row")
            .len();

        Self {
            backing: input.as_bytes(),
            height,
            width,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.backing[y * (self.width + 1) + x]
    }
}

const fn coords_to_idx(board: &Board, x: usize, y: usize) -> usize {
    x + (y * board.width)
}

pub fn do_aoc(input: &str) -> usize {
    let board = Board::new(input);
    let mut visited = bitvec![0; board.height*board.width];

    // flood fill stack
    let mut ffs = vec![];

    // result
    let mut total = 0;
    
    for i in 0..board.height * board.width {
        if visited[i] {
            continue;
        }

        ffs.clear();

        let mut area = 0;
        let mut perimeter = 0;
        let (sx, sy) = (i % board.width, i / board.width);
        let plant = board.get(sx, sy);
        ffs.push((sx, sy));

        //println!("Region {} start", plant as char);

        while let Some((x, y)) = ffs.pop() {
            if visited[coords_to_idx(&board, x, y)] {
                continue;
            }

            //println!(" visiting {x},{y}");

            area += 1;
            visited.set((y * board.width) + x, true);
            // West
            //println!("  west ({},{})", x as isize - 1,y as isize);
            if x == 0 || board.get(x - 1, y) != plant {
                //println!("   inc perim");
                perimeter += 1 
            } else if !visited[coords_to_idx(&board, x - 1, y)] {
                //println!("   add to visit");
                ffs.push((x - 1, y));
            }

            // East
            //println!("  east ({},{})", x as isize + 1,y as isize);
            if x + 1 >= board.width || board.get(x + 1, y) != plant {
                //println!("   inc perim");
                perimeter += 1
            } else if !visited[coords_to_idx(&board, x + 1, y)] {
                //println!("   add to visit");
                ffs.push((x + 1, y));
            }

            //println!("  north ({},{})", x as isize,y as isize - 1);
            // North
            if y == 0 || board.get(x, y - 1) != plant {
                //println!("   inc perim");
                perimeter += 1
            } else if !visited[coords_to_idx(&board, x, y - 1)] {
                //println!("   add to visit");
                ffs.push((x, y - 1));
            }

            //println!("  south ({},{})", x as isize,y as isize + 1);
            // South
            if y + 1 >= board.height || board.get(x, y + 1) != plant {
                //println!("   inc perim");
                perimeter += 1
            } else if !visited[coords_to_idx(&board, x, y + 1)] {
                //println!("   add to visit");
                ffs.push((x, y + 1));
            }
        }

        //println!("Region {} has perim:{perimeter} area:{area} price:{}",plant as char, perimeter*area);

        total += perimeter * area
    }

    total
}
