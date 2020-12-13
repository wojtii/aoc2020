use std::fs::read_to_string;

const PREAMBLE: usize = 25;
pub fn run() {
    let input = read_to_string("src/day9-input.txt").unwrap();
    let nums: Vec<u64> = input.lines().filter_map(|x| x.parse().ok()).collect();

    let result = first_no_sum(&nums);
    assert_eq!(1212510616, result);
    println!("{}", result);

    let invalid_number = result;
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    let result;
    loop {
        sum += nums[j];
        if sum == invalid_number {
            let min = nums[i + 1..j].iter().min().unwrap();
            let max = nums[i + 1..j].iter().max().unwrap();
            result = min + max;
            break;
        }

        if sum > invalid_number {
            i += 1;
            j = i;
            sum = 0;
        }

        j += 1;
    }

    assert_eq!(171265123, result);
    println!("{}", result);
}

fn first_no_sum(nums: &[u64]) -> u64 {
    for (i, num) in nums[PREAMBLE..].iter().enumerate() {
        if !has_sum(num, &nums[i..i + PREAMBLE]) {
            return *num;
        }
    }

    unreachable!();
}

fn has_sum(num: &u64, prevs: &[u64]) -> bool {
    for n1 in prevs.iter() {
        for n2 in prevs.iter() {
            if n1 + n2 == *num {
                return true;
            }
        }
    }

    false
}
