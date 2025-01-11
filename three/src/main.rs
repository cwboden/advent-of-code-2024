use advent::prelude::*;

#[derive(Debug, HasParser)]
enum Action {
    #[parse(before = "mul(", after = ")", sep_by = ",")]
    Mul(u32, u32),
    #[parse(string = "do()")]
    Do,
    #[parse(string = "don't()")]
    Dont,
}

#[derive(Debug)]
struct MulList(Vec<Action>);

impl HasParser for MulList {
    #[into_parser]
    fn parser() -> _ {
        many1(attempt(Action::parser().map(Some)).or(any().map(|_| None)))
            .map(|m: Vec<_>| Self(m.into_iter().flatten().collect()))
    }
}

#[part_one]
fn part_one(input: MulList) -> u32 {
    input
        .0
        .into_iter()
        .map(|a| match a {
            Action::Mul(i, j) => i * j,
            _ => 0,
        })
        .sum()
}

#[part_two]
fn part_two(input: MulList) -> u32 {
    let mut enabled = true;
    input
        .0
        .into_iter()
        .map(|a| match a {
            Action::Mul(i, j) => enabled.then(|| i * j).unwrap_or(0),
            Action::Do => {
                enabled = true;
                0
            }
            Action::Dont => {
                enabled = false;
                0
            }
        })
        .sum()
}

harness!(part_1: 159892596, part_2: 92626942);
