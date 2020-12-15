use std::collections::HashMap;

pub fn run() {
    let input = "16,1,0,18,12,14,19";
    let nums: Vec<u32> = input.split(",").map(|x| x.parse().unwrap()).collect();

    let result = nth(&nums, 2020);
    assert_eq!(929, result);
    println!("{}", result);

    let result = nth(&nums, 30000000);
    assert_eq!(16671510, result);
    println!("{}", result);
}

fn nth(nums: &Vec<u32>, upper: usize) -> u32 {
    let mut last_turn: HashMap<u32, usize> =
        nums.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    (nums.len()..upper).fold(*nums.last().unwrap(), |prev, turn| {
        let next = if let Some(v) = last_turn.get(&prev) {
            turn - v - 1
        } else {
            0
        };
        last_turn.insert(prev, turn - 1);
        next as u32
    })
}
