use advent::prelude::*;

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(l, r)| l - r)
        .collect();

    diffs.iter().all(|i| (1..=3).contains(i)) || diffs.iter().all(|i| (-3..=-1).contains(i))
}

#[part_one]
fn part_one(input: List<List<i32, SepBy<Space>>, TermWith<NewLine>>) -> usize {
    input.into_iter().filter(|r| is_safe(r)).count()
}

#[part_two]
fn part_two(input: List<List<i32, SepBy<Space>>, TermWith<NewLine>>) -> usize {
    let (safe_reports, unsafe_reports): (
        Vec<List<i32, SepBy<Space>>>,
        Vec<List<i32, SepBy<Space>>>,
    ) = input.into_iter().partition(|r| is_safe(r));

    safe_reports.into_iter().count()
        + unsafe_reports
            .into_iter()
            .filter(|r| {
                (0..r.len()).any(|i| {
                    let mut with_one_removal = r.to_vec();
                    with_one_removal.remove(i);
                    is_safe(&with_one_removal)
                })
            })
            .count()
}

harness!(part_1: 606, part_2: 644);
