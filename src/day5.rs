use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("src/day5-input.txt").unwrap();
    let ids: HashSet<_> = input.lines().map(|x| seat_to_id(x)).collect();

    let max_id = ids.iter().max().unwrap();
    assert_eq!(&835, max_id);
    println!("{}", max_id);

    let my_id = ids.iter().fold(0, |acc, x| {
        if ids.contains(&(x + 2)) && !ids.contains(&(x + 1)) {
            x + 1
        } else {
            acc
        }
    });
    assert_eq!(649, my_id);
    println!("{}", my_id);
}

fn seat_to_id(seat: &str) -> u32 {
    let from_bin = |x| u32::from_str_radix(x, 2).unwrap();
    let bin_seats: String = seat
        .chars()
        .map(|x| match x {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => unreachable!(),
        })
        .collect();
    let (row, column) = bin_seats.split_at(7);
    from_bin(row) * 8 + from_bin(column)
}
