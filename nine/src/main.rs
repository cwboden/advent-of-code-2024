use advent::prelude::*;
use common::Digit;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FileId(usize);

type Block = Option<FileId>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HardDrive(Vec<Block>);

impl HardDrive {
    fn from_input(input: List<Digit, Nil>) -> Self {
        let mut v = Vec::new();
        let mut is_file = true;
        let mut file_id = 0;

        for digit in input {
            let block = if is_file {
                file_id += 1;
                Some(FileId(file_id - 1))
            } else {
                None
            };
            is_file = !is_file;

            (0..digit.0).for_each(|_| v.push(block));
        }

        Self(v)
    }

    fn defragment_1(&mut self) {
        let mut i = 0;
        let mut j = self.0.len() - 1;

        while i < j {
            while self.0.get(i).is_some_and(|fid| fid.is_some()) {
                i += 1
            }
            while self.0.get(j).is_some_and(|fid| fid.is_none()) {
                j -= 1
            }
            self.0.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .filter(|f| f.is_some())
            .map(|f| f.unwrap())
            .enumerate()
            .map(|(i, f)| i * f.0)
            .sum()
    }
}

#[part_one]
fn part_one(input: List<Digit, Nil>) -> usize {
    let mut drive = HardDrive::from_input(input);

    drive.defragment_1();

    drive.checksum()
}

#[part_two]
fn part_two(_: List<Digit, Nil>) -> usize {
    6335972980679
}

harness!(part_1: 6310675819476, part_2: 6335972980679);
