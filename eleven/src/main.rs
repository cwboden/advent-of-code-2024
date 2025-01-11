use advent::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Stone(u128);

impl Stone {
    fn blink(self) -> [Option<Self>; 2] {
        let value = self.0.to_string();
        if value == "0" {
            [Some(Stone(1)), None]
        } else if value.len() % 2 == 0 {
            // has an even number of digits
            let (s1, s2) = value.split_at(value.len() / 2);
            [
                Some(Stone(s1.parse().unwrap())),
                Some(Stone(s2.parse().unwrap())),
            ]
        } else {
            [Some(Stone(self.0 * 2024)), None]
        }
    }
}

fn from_input(input: List<u128, SepBy<Space>>) -> HashMap<Stone, usize> {
    let mut stones = HashMap::new();
    for s in input {
        *stones.entry(Stone(s)).or_default() += 1;
    }
    stones
}

fn blink_times(stones: HashMap<Stone, usize>, times: u32) -> HashMap<Stone, usize> {
    (0..times).into_iter().fold(stones, |stones, _| {
        let mut new_stones = HashMap::new();

        for (stone, count) in stones {
            stone
                .blink()
                .into_iter()
                .filter_map(|s| s)
                .for_each(|s| *new_stones.entry(s).or_default() += count);
        }

        new_stones
    })
}

#[part_one]
fn part_one(input: List<u128, SepBy<Space>>) -> usize {
    let stones = from_input(input);
    blink_times(stones, 25).values().sum()
}

#[part_two]
fn part_two(input: List<u128, SepBy<Space>>) -> usize {
    let stones = from_input(input);
    blink_times(stones, 75).values().sum()
}

harness!(part_1: 217443, part_2: 257246536026785);
