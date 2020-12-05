use std::fs::read_to_string;

const TREE: char = '#';

pub fn run() {
    let input = read_to_string("src/day3-input.txt").unwrap();
    let map: Vec<_> = input.lines().collect();

    let step_three = traverse(&map, 3, 1);
    assert_eq!(244, step_three);
    println!("{}", step_three);

    let step_one = traverse(&map, 1, 1);
    assert_eq!(90, step_one);

    let step_five = traverse(&map, 5, 1);
    assert_eq!(97, step_five);

    let step_seven = traverse(&map, 7, 1);
    assert_eq!(92, step_seven);

    let step_one_two = traverse(&map, 1, 2);
    assert_eq!(48, step_one_two);

    let result = step_one * step_three * step_five * step_seven * step_one_two;
    println!("{}", result);
    assert_eq!(9406609920, result);
}

fn traverse(map: &Vec<&str>, step_side: usize, step_down: usize) -> i64 {
    let next_idx_side = |idx_step_sum, len| {
        if idx_step_sum >= len {
            idx_step_sum - len
        } else {
            idx_step_sum
        }
    };
    let next_idx_down = |idx| idx + 1;

    let (_, _, result) = map.iter().fold((0, 0, 0), |(idx_side, idx_down, cnt), x| {
        if idx_down % step_down != 0 {
            (idx_side, next_idx_down(idx_down), cnt)
        } else {
            (
                next_idx_side(idx_side + step_side, x.len()),
                next_idx_down(idx_down),
                match x.chars().nth(idx_side) {
                    Some(TREE) => cnt + 1,
                    Some(_) => cnt,
                    None => cnt,
                },
            )
        }
    });

    result
}
