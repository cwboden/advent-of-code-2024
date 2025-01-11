use advent::prelude::*;
use common::Coord;
use common::Dir;
use common::GetCoord;

// Note: Remember path-finding algo from personal repo!

#[derive(Debug, HasParser)]
#[parse(sep_by = ",")]
struct InputCoord(usize, usize);

impl Into<Coord> for InputCoord {
    fn into(self) -> Coord {
        Coord(self.0, self.1)
    }
}

#[part_one]
fn part_one(input: List<InputCoord, TermWith<NewLine>>) -> usize {
    let bytes_fallen = 1024; // 1KiB
    let fallen_bytes: HashSet<Coord> = input
        .into_iter()
        .take(bytes_fallen)
        .map(|i| i.into())
        .collect();

    let start = Coord(0, 0);
    let goal = Coord(70, 70);
    let bounds = goal;

    assert!(!fallen_bytes.contains(&goal));

    println!("loaded:\n");
    for x in start.0..goal.0 {
        for y in start.1..goal.1 {
            if fallen_bytes.contains(&Coord(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    let mut spaces_traversed = Grid::<usize>::new(vec![vec![77 * 77 * 77; 71]; 71]).unwrap();
    spaces_traversed[start.0][start.1] = 0;
    let mut spaces_queue = VecDeque::new();
    spaces_queue.push_back(start);

    while let Some(space) = spaces_queue.pop_front() {
        let current_moves = spaces_traversed.get_coord(space).unwrap();
        let next_moves = current_moves + 1;
        for dir in Dir::iter() {
            if let Some(next_coord) = dir.advance(space, bounds) {
                if fallen_bytes.contains(&next_coord) {
                    // can't move through fallen bytes
                    continue;
                } else if spaces_traversed
                    .get_coord(next_coord)
                    .is_some_and(|min_moves| *min_moves < next_moves)
                {
                    // no point exploring paths that are faster than us
                    continue;
                } else if next_coord == goal {
                    // we found it!
                    print!("we found it!");
                    continue;
                }

                // print!("visiting {:?}, ", next_coord);
                spaces_queue.push_back(next_coord);
                spaces_traversed[next_coord.0][next_coord.1] = next_moves;
            }
        }
    }

    *spaces_traversed.get_coord(goal).unwrap()
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
