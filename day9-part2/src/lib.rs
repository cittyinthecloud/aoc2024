#[derive(Clone, Copy, Debug)]
struct FileBlock {
    location: u64,
    count: u8,
    total_free_space: u8,
    consumed_free_space: u8,
}

fn calc_csum(idx: u64, count: u64, id: u64) -> u64 {
    (count * id * ((count - 1) + (2 * idx))) / 2
}

pub fn do_aoc(input: &str) -> u64 {
    let mut cursor = 0;
    let mut file_blocks: Vec<_> = input
        .as_bytes()
        .trim_ascii_end()
        .chunks(2)
        .map(|chunk| {
            let count = chunk[0] - b'0';
            let free_space = if let Some(x) = chunk.get(1) {
                x - b'0'
            } else {
                0
            };

            let fb = FileBlock {
                location: cursor,
                count,
                total_free_space: free_space,
                consumed_free_space: 0,
            };

            cursor += count as u64 + free_space as u64;

            fb
        })
        .collect();

    // println!("{file_blocks:?}");
    let mut checksum: u64 = 0;

    for end_block_id in (0..file_blocks.len()).rev() {
        // println!("Trying block {:?}", file_blocks[end_block_id]);

        let mut found = false;

        let file_size: u8 = file_blocks[end_block_id].count;

        for start_block_id in 0..end_block_id {
            let start_block = &mut file_blocks[start_block_id];
            let space_left = start_block.total_free_space - start_block.consumed_free_space;

            // if this fits into start_block
            if file_size <= space_left {
                // count blocks are used by the file, consumed blocks we've already used to fit other files.
                let cur_block = start_block.location
                    + start_block.count as u64
                    + start_block.consumed_free_space as u64;

                // println!("Block fits in {start_block:?}, consuming {file_size} at {cur_block}");
                checksum += calc_csum(cur_block, file_size as u64, end_block_id as u64);
                start_block.consumed_free_space += file_size;
                found = true;
                break;
            }
        }

        if !found {
            // println!("Block doesn't fit in any block, adding to checksum and moving on...");
            checksum += calc_csum(
                file_blocks[end_block_id].location,
                file_size as u64,
                end_block_id as u64,
            );
        }
    }

    return checksum;
}
