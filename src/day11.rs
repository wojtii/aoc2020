use std::fs::read_to_string;

type Decider = fn(&Vec<Vec<char>>, usize, usize) -> bool;

const TAKEN: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

const ADJACENT_SEATS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

pub fn run() {
    let input: Vec<Vec<char>> = read_to_string("src/day11-input.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_owned().chars().collect::<Vec<char>>())
        .collect();

    let occupy_empty: Decider = |layout, i, j| -> bool { count_adjacent_taken(layout, i, j) == 0 };
    let empty_taken: Decider = |layout, i, j| -> bool { count_adjacent_taken(layout, i, j) > 3 };
    let layout = find_stable(&input, occupy_empty, empty_taken);
    let result = count_taken(&layout);
    assert_eq!(2204, result);
    println!("{}", result);

    let occupy_empty: Decider = |layout, i, j| -> bool { count_seen_taken(layout, i, j) == 0 };
    let empty_taken: Decider = |layout, i, j| -> bool { count_seen_taken(layout, i, j) > 4 };
    let layout = find_stable(&input, occupy_empty, empty_taken);
    let result = count_taken(&layout);
    assert_eq!(1986, result);
    println!("{}", result);
}

fn count_adjacent_taken(layout: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    ADJACENT_SEATS.iter().fold(0, |acc, x| {
        let (ii, jj) = (i as i32 + x.0, j as i32 + x.1);
        if ii > 0 || jj > 0 {
            let (ii, jj) = (ii as usize, jj as usize);
            if ii < layout.len() && jj < layout[ii].len() && layout[ii][jj] == TAKEN {
                return acc + 1;
            }
        }
        acc
    })
}

fn count_seen_taken(layout: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    ADJACENT_SEATS.iter().fold(0, |acc, x| {
        let (mut ii, mut jj) = (i as i32, j as i32);
        loop {
            ii += x.0;
            jj += x.1;
            if ii < 0 || jj < 0 {
                break;
            }
            let (ii, jj) = (ii as usize, jj as usize);

            if ii >= layout.len() || jj >= layout[ii].len() || layout[ii][jj] == EMPTY {
                break;
            }

            if layout[ii][jj] == TAKEN {
                return acc + 1;
            }
        }

        acc
    })
}

fn find_stable(
    layout: &Vec<Vec<char>>,
    occupy_empty: Decider,
    empty_taken: Decider,
) -> Vec<Vec<char>> {
    let mut next = Vec::with_capacity(layout.len());
    let mut changed = false;

    for (i, row) in layout.iter().enumerate() {
        let mut new_row = Vec::with_capacity(row.len());
        for (j, seat) in row.iter().enumerate() {
            let new_seat = match *seat {
                FLOOR => FLOOR,
                EMPTY => {
                    if occupy_empty(layout, i, j) {
                        TAKEN
                    } else {
                        EMPTY
                    }
                }
                TAKEN => {
                    if empty_taken(layout, i, j) {
                        EMPTY
                    } else {
                        TAKEN
                    }
                }
                x => unreachable!(format!("unexpected '{:?}'", x)),
            };
            if seat != &new_seat {
                changed = true;
            }
            new_row.push(new_seat);
        }
        next.push(new_row);
    }

    if changed {
        find_stable(&next, occupy_empty, empty_taken)
    } else {
        next
    }
}

fn count_taken(layout: &Vec<Vec<char>>) -> usize {
    layout.iter().flatten().filter(|&x| *x == TAKEN).count()
}
