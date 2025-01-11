use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = "   ")]
struct Input {
    left: u32,
    right: u32,
}

#[part_one]
fn part_one(input: List<Input, TermWith<NewLine>>) -> u32 {
    let (mut left_list, mut right_list): (Vec<u32>, Vec<u32>) =
        input.iter().map(|i| (i.left, i.right)).unzip();

    left_list.sort();
    right_list.sort();

    left_list
        .into_iter()
        .zip(right_list)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[part_two]
fn part_two(input: List<Input, TermWith<NewLine>>) -> u32 {
    let (left_list, right_list): (Vec<u32>, Vec<u32>) =
        input.iter().map(|i| (i.left, i.right)).unzip();

    fn count(list: Vec<u32>) -> HashMap<u32, u32> {
        list.into_iter().fold(HashMap::new(), |mut m, i| {
            *m.entry(i).or_default() += 1;
            m
        })
    }

    let right_counts = count(right_list);
    count(left_list)
        .into_iter()
        .map(|(i, left_count)| right_counts.get(&i).cloned().unwrap_or_default() * i * left_count)
        .sum()
}

harness!(part_1: 1197984);
