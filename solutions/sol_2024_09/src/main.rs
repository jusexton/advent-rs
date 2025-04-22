#[derive(PartialEq, Eq, Debug)]
struct FileBlock {
    size: usize,
    idx: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct DefragmentedDiskMap {
    raw: Vec<i32>,
}

impl DefragmentedDiskMap {
    fn checksum(&self) -> u64 {
        self.raw
            .iter()
            .enumerate()
            .filter(|(_, bit)| **bit != -1)
            .map(|(idx, &bit)| idx as u64 * bit as u64)
            .sum()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct DiskMap {
    raw: Vec<i32>,
    occupied_blocks: Vec<FileBlock>,
    empty_blocks: Vec<FileBlock>,
}

impl DiskMap {
    fn new(input: &str) -> Self {
        let mut raw = Vec::new();
        let mut occupied_blocks = Vec::new();
        let mut empty_blocks = Vec::new();
        let mut is_file = true;
        let mut id = 0;
        for c in input.chars() {
            let size = c.to_digit(10).unwrap() as usize;
            match is_file {
                true if size > 0 => {
                    occupied_blocks.push(FileBlock {
                        size,
                        idx: raw.len(),
                    });
                    raw.extend(vec![id; size]);
                    id += 1;
                }
                false if size > 0 => {
                    empty_blocks.push(FileBlock {
                        size,
                        idx: raw.len(),
                    });
                    raw.extend(vec![-1; size]);
                }
                _ => {}
            }
            is_file = !is_file;
        }
        Self {
            raw,
            empty_blocks,
            occupied_blocks,
        }
    }

    fn into_defrag_by_bits(mut self) -> DefragmentedDiskMap {
        let n = self.len() - 1;
        let (mut left, mut right) = (0, n);
        while left <= right {
            // Find empty space that needs to be filled
            while self.raw[left] != -1 {
                left += 1
            }
            // Find right bits that need to be swapped
            while self.raw[right] == -1 {
                right -= 1;
            }
            self.raw.swap(left, right);
        }
        self.raw.swap(left, right);
        DefragmentedDiskMap { raw: self.raw }
    }

    fn into_defrag_by_chunks(mut self) -> DefragmentedDiskMap {
        while let Some(occupied_block) = self.occupied_blocks.pop() {
            // Match with first empty block that can fit the occupied block
            if let Some(empty_block) = self
                .empty_blocks
                .iter_mut()
                .find(|block| block.size >= occupied_block.size && block.idx < occupied_block.idx)
            {
                // Swap bits
                for i in 0..occupied_block.size {
                    self.raw.swap(empty_block.idx + i, occupied_block.idx + i);
                }

                // Update empty chunk
                empty_block.idx += occupied_block.size;
                empty_block.size -= occupied_block.size;
            }
        }
        DefragmentedDiskMap { raw: self.raw }
    }

    fn len(&self) -> usize {
        self.raw.len()
    }
}

pub fn defrag_file_bits_and_checksum(input: &str) -> u64 {
    let defragmented = DiskMap::new(input).into_defrag_by_bits();
    defragmented.checksum()
}

pub fn defrag_file_chunk_and_checksum(input: &str) -> u64 {
    let defragmented = DiskMap::new(input).into_defrag_by_chunks();
    defragmented.checksum()
}

fn main() {
    let input = include_str!("../data.txt");
    println!(
        "(2024 - Day 9) --- {}, {}",
        defrag_file_bits_and_checksum(input),
        defrag_file_chunk_and_checksum(input)
    );
}
