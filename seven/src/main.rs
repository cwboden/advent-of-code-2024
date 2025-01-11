use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = ": ")]
struct Equation {
    result: u64,
    operands: List<u64, SepBy<Space>>,
}

#[derive(Debug, HasParser)]
struct Input {
    equations: List<Equation, TermWith<NewLine>>,
}

fn can_be_evaluated(result: u64, current: u64, operands: &[u64], part_2: bool) -> bool {
    if operands.is_empty() {
        return result == current;
    } else if result < current {
        // we assume the number never goes down
        return false;
    }

    let next_operand: u64 = operands.first().unwrap().clone().into();
    assert_ne!(next_operand, 0); // validates our early return above
    return can_be_evaluated(result, current + next_operand, &operands[1..], part_2)
        || can_be_evaluated(result, current * next_operand, &operands[1..], part_2)
        || (part_2
            && can_be_evaluated(
                result,
                (current.to_string() + &next_operand.to_string())
                    .parse::<u64>()
                    .unwrap(),
                &operands[1..],
                true,
            ));
}

#[part_one]
fn part_one(input: Input) -> u64 {
    let results = input
        .equations
        .into_iter()
        .filter(|e| can_be_evaluated(e.result, 0, &e.operands, false));

    results.map(|e| e.result).sum()
}

#[part_two]
fn part_two(input: Input) -> u64 {
    let results = input
        .equations
        .into_iter()
        .filter(|e| can_be_evaluated(e.result, 0, &e.operands, true));

    results.map(|e| e.result).sum()
}

harness!(part_1: 42283209483350, part_2: 1026766857276279);
