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
            .filter(|(_, &bit)| bit != -1)
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
            while self.raw[left] != -1 {
                left += 1
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_disk_map_from_input() {
        assert_eq!(
            DiskMap {
                raw: vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2],
                occupied_blocks: vec![
                    FileBlock { idx: 0, size: 1 },
                    FileBlock { idx: 3, size: 3 },
                    FileBlock { idx: 10, size: 5 }
                ],
                empty_blocks: vec![FileBlock { idx: 1, size: 2 }, FileBlock { idx: 6, size: 4 }],
            },
            DiskMap::new("12345")
        );

        assert_eq!(
            DiskMap {
                raw: vec![
                    0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5,
                    5, 5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
                ],
                occupied_blocks: vec![
                    FileBlock { idx: 0, size: 2 },
                    FileBlock { idx: 5, size: 3 },
                    FileBlock { idx: 11, size: 1 },
                    FileBlock { idx: 15, size: 3 },
                    FileBlock { idx: 19, size: 2 },
                    FileBlock { idx: 22, size: 4 },
                    FileBlock { idx: 27, size: 4 },
                    FileBlock { idx: 32, size: 3 },
                    FileBlock { idx: 36, size: 4 },
                    FileBlock { idx: 40, size: 2 }
                ],
                empty_blocks: vec![
                    FileBlock { idx: 2, size: 3 },
                    FileBlock { idx: 8, size: 3 },
                    FileBlock { idx: 12, size: 3 },
                    FileBlock { idx: 18, size: 1 },
                    FileBlock { idx: 21, size: 1 },
                    FileBlock { idx: 26, size: 1 },
                    FileBlock { idx: 31, size: 1 },
                    FileBlock { idx: 35, size: 1 },
                ],
            },
            DiskMap::new("2333133121414131402")
        );
    }

    #[test]
    fn defragments_using_file_bit_strategy() {
        let disk_map = DiskMap {
            raw: vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
            ],
            occupied_blocks: vec![
                FileBlock { idx: 0, size: 2 },
                FileBlock { idx: 5, size: 3 },
                FileBlock { idx: 11, size: 1 },
                FileBlock { idx: 15, size: 3 },
                FileBlock { idx: 19, size: 2 },
                FileBlock { idx: 22, size: 4 },
                FileBlock { idx: 27, size: 4 },
                FileBlock { idx: 32, size: 3 },
                FileBlock { idx: 36, size: 4 },
                FileBlock { idx: 40, size: 2 },
            ],
            empty_blocks: vec![
                FileBlock { idx: 2, size: 3 },
                FileBlock { idx: 8, size: 3 },
                FileBlock { idx: 12, size: 3 },
                FileBlock { idx: 18, size: 1 },
                FileBlock { idx: 21, size: 1 },
                FileBlock { idx: 26, size: 1 },
                FileBlock { idx: 31, size: 1 },
                FileBlock { idx: 35, size: 1 },
            ],
        };

        assert_eq!(
            DefragmentedDiskMap {
                raw: vec![
                    0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5,
                    6, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
                ]
            },
            disk_map.into_defrag_by_bits()
        );
    }

    #[test]
    fn defragments_using_file_chunk_strategy() {
        let disk_map = DiskMap {
            raw: vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
            ],
            occupied_blocks: vec![
                FileBlock { idx: 0, size: 2 },
                FileBlock { idx: 5, size: 3 },
                FileBlock { idx: 11, size: 1 },
                FileBlock { idx: 15, size: 3 },
                FileBlock { idx: 19, size: 2 },
                FileBlock { idx: 22, size: 4 },
                FileBlock { idx: 27, size: 4 },
                FileBlock { idx: 32, size: 3 },
                FileBlock { idx: 36, size: 4 },
                FileBlock { idx: 40, size: 2 },
            ],
            empty_blocks: vec![
                FileBlock { idx: 2, size: 3 },
                FileBlock { idx: 8, size: 3 },
                FileBlock { idx: 12, size: 3 },
                FileBlock { idx: 18, size: 1 },
                FileBlock { idx: 21, size: 1 },
                FileBlock { idx: 26, size: 1 },
                FileBlock { idx: 31, size: 1 },
                FileBlock { idx: 35, size: 1 },
            ],
        };

        assert_eq!(
            DefragmentedDiskMap {
                raw: vec![
                    0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5,
                    5, 5, -1, 6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1
                ]
            },
            disk_map.into_defrag_by_chunks()
        );
    }

    #[test]
    fn calculates_checksum() {
        let disk_map = DefragmentedDiskMap {
            raw: vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2],
        };
        assert_eq!(132, disk_map.checksum())
    }
}
