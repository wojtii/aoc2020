use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("src/day13-input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    assert_eq!(2, lines.len());
    let timestamp: usize = lines[0].parse().unwrap();
    let buses: Vec<(usize, usize)> = lines[1]
        .split(",")
        .enumerate()
        .filter_map(|(i, x)| match x.parse() {
            Err(_) => None,
            Ok(v) => Some((i, v)),
        })
        .collect();

    let (idx, minutes_diff) = buses
        .iter()
        .enumerate()
        .map(|(i, (_, x))| (i, x - timestamp.rem_euclid(*x)))
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();
    let result = minutes_diff * buses[idx].1;
    assert_eq!(2305, result);
    println!("{}", result);

    let (result, _) = buses
        .iter()
        .fold((0, 1), |(result, product_so_far), (minutes, id)| {
            let mut next_result = result;
            loop {
                next_result += product_so_far;
                if (next_result + minutes).rem_euclid(*id) == 0 {
                    return (next_result, product_so_far * id);
                }
            }
        });
    assert_eq!(552612234243498, result);
    println!("{}", result);
}
