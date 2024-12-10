#[derive(Clone, Copy, Debug)]
struct FileBlock {
    count: u8,
    free_space_after: u8
}

fn calc_csum(idx: u64, count: u64, id: u64) -> u64 {
    (count * id * ((count - 1) + (2 * idx))) / 2
}

pub fn do_aoc(input: &str) -> u64 {
    let mut file_blocks: Vec<_> = input
        .as_bytes()
        .trim_ascii_end()
        .chunks(2)
        .map(|chunk| FileBlock { 
            count: chunk[0] - b'0',
            free_space_after: if let Some(x) = chunk.get(1) {
                x - b'0'
            } else {
                0
            },
        })
        .collect();

    let mut checksum: u64 = 0;
    let mut end_block_idx = file_blocks.len() - 1;

    let mut cur_block: u64 = 0;

    let mut was_partial = false;

    for block_id in 0..file_blocks.len() {
        if block_id >= end_block_idx {
            break;
        }

        let block = file_blocks[block_id];

        // println!("Loop at {cur_block} : {i} {end_block_idx} : {block:?} {:?}", file_blocks[end_block_idx]);
        let block_count: u64 = block.count as u64;

        checksum += calc_csum(cur_block, block_count, block_id as u64);

        cur_block += block_count;

        // println!("Loop csum: {checksum}");

        let mut free_space_to_fill: u8 = block.free_space_after;

        while free_space_to_fill > 0 {
            let end_block_space_needed = file_blocks[end_block_idx].count;

            // println!("Have {free_space_to_fill}, Filling with {:?}", file_blocks[end_block_idx]);

            let end_block = file_blocks[end_block_idx];

            if end_block_space_needed <= free_space_to_fill {
                // println!("Fitting in entire block at {cur_block}");

                checksum += calc_csum(cur_block, end_block_space_needed as u64, end_block_idx as u64);
                cur_block += end_block_space_needed as u64;
                free_space_to_fill -= end_block_space_needed;
                end_block_idx -= 1;
                was_partial = true;
            } else {
                file_blocks[end_block_idx].count -= free_space_to_fill;

                // println!("Fitting in partial block at {cur_block}, left with {:?}",file_blocks[end_block_idx]);

                checksum += calc_csum(cur_block, free_space_to_fill as u64, end_block_idx as u64);
                cur_block += free_space_to_fill as u64;

                // println!("Fill csum: {checksum}");
                was_partial = true;
                break;
            }

            // println!("Fill csum: {checksum}")
        }
    }

    if was_partial {
        // Our last block has some left over at the end
        checksum += calc_csum(
            cur_block,
            file_blocks[end_block_idx].count as u64,
            end_block_idx as u64,
        );
    }

    return checksum;
}
