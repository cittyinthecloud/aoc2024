use bitvec::prelude::*;
use std::num::NonZeroU8;

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn do_aoc(input: &str) -> usize {
    let board = Board::new(input);
    let mut visited = bitvec![0; board.height*board.width];

    // flood fill stack
    let mut ffs = vec![];
    let mut perimeter_edges = vec![];
    // shape_cells
    // result
    let mut total = 0;
    
    for i in 0..board.height * board.width {
        if visited[i] {
            continue;
        }

        ffs.clear();

        let mut area = 0;
        // let mut perimeter = 0;
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
                perimeter_edges.push((x,y,Direction::West)); 
            } else if !visited[coords_to_idx(&board, x - 1, y)] {
                //println!("   add to visit");
                ffs.push((x - 1, y));
            }

            // East
            //println!("  east ({},{})", x as isize + 1,y as isize);
            if x + 1 >= board.width || board.get(x + 1, y) != plant {
                //println!("   inc perim");
                perimeter_edges.push((x,y,Direction::East)); 
            } else if !visited[coords_to_idx(&board, x + 1, y)] {
                //println!("   add to visit");
                ffs.push((x + 1, y));
            }

            //println!("  north ({},{})", x as isize,y as isize - 1);
            // North
            if y == 0 || board.get(x, y - 1) != plant {
                //println!("   inc perim");
                perimeter_edges.push((x,y,Direction::North)); 
            } else if !visited[coords_to_idx(&board, x, y - 1)] {
                //println!("   add to visit");
                ffs.push((x, y - 1));
            }

            //println!("  south ({},{})", x as isize,y as isize + 1);
            // South
            if y + 1 >= board.height || board.get(x, y + 1) != plant {
                //println!("   inc perim");
                perimeter_edges.push((x,y,Direction::South)); 
            } else if !visited[coords_to_idx(&board, x, y + 1)] {
                //println!("   add to visit");
                ffs.push((x, y + 1));
            }
        }

        // Now count the unique edges
        let mut unique_edges = 0;
        while let Some((x, y, dir)) = perimeter_edges.pop() {
            unique_edges += 1;
            match dir {
                Direction::North | Direction::South => {
                    for x1 in x+1.. {
                        if let Some(index) = perimeter_edges.iter().position(|&pos| pos == (x1, y, dir)) {
                            perimeter_edges.swap_remove(index);
                        } else {
                            break;
                        }
                    }
                    for x1 in (0..x).rev() {
                        if let Some(index) = perimeter_edges.iter().position(|&pos| pos == (x1, y, dir)) {
                            perimeter_edges.swap_remove(index);
                        } else {
                            break;
                        }
                    }
                },
                Direction::East | Direction::West => {
                    for y1 in y+1.. {
                        if let Some(index) = perimeter_edges.iter().position(|&pos| pos == (x, y1, dir)) {
                            perimeter_edges.swap_remove(index);
                        } else {
                            break;
                        }
                    }
                    for y1 in (0..y).rev() {
                        if let Some(index) = perimeter_edges.iter().position(|&pos| pos == (x, y1, dir)) {
                            perimeter_edges.swap_remove(index);
                        } else {
                            break;
                        }
                    }
                },
            }
        }
    
        //println!("Region {} has perim:{perimeter} area:{area} price:{}",plant as char, perimeter*area);

        total += area * unique_edges
    }

    total
}
