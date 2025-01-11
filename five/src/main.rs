use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = "|")]
struct PageOrdering(u32, u32);

#[derive(Debug, HasParser)]
#[parse(sep_by = "\n")]
struct Input {
    orderings: List<PageOrdering, TermWith<NewLine>>,
    updates: List<List<u32, SepBy<Comma>>, TermWith<NewLine>>,
}

fn is_valid(requirements: &[PageOrdering], update: &[u32]) -> bool {
    let indices: HashMap<_, _> = update.iter().enumerate().map(|(i, p)| (p, i)).collect();

    requirements
        .iter()
        .filter(|ord| indices.contains_key(&ord.0) && indices.contains_key(&ord.1))
        .all(|ord| indices[&ord.0] < indices[&ord.1])
}

#[part_one]
fn part_one(input: Input) -> u32 {
    let valid_updates = input
        .updates
        .into_iter()
        .filter(|u| is_valid(&input.orderings, u));
    valid_updates
        .map(|u| u.get(u.len() / 2).unwrap().clone())
        .sum()
}

#[part_two]
fn part_two(input: Input) -> u32 {
    let invalid_updates = input
        .updates
        .into_iter()
        .filter(|u| !is_valid(&input.orderings, u));

    let sorted_updates = invalid_updates.map(|mut u| {
        let mut indices: HashMap<_, _> =
            u.iter().cloned().enumerate().map(|(i, p)| (p, i)).collect();

        while !is_valid(&input.orderings, &u) {
            for ordering in &input.orderings {
                if !indices.contains_key(&ordering.0) || !indices.contains_key(&ordering.1) {
                    continue;
                }

                let index_0 = indices[&ordering.0];
                let index_1 = indices[&ordering.1];
                if index_0 > index_1 {
                    u.swap(index_0, index_1);
                    indices.insert(ordering.1, index_0);
                    indices.insert(ordering.0, index_1);
                }
            }
        }
        u
    });

    sorted_updates
        .map(|u| u.get(u.len() / 2).unwrap().clone())
        .sum()
}

harness!(part_1: 4609, part_2: 5723);
