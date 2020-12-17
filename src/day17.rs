use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::FromIterator;
use std::ops::Range;
use std::ops::RangeInclusive;

const ACTIVE: char = '#';
const CORDS: RangeInclusive<i32> = -1..=1;
const CYCLES: Range<u8> = 0..6;

pub fn run() {
    let input = read_to_string("src/day17-input.txt").unwrap();
    let grid: Vec<_> = input.lines().collect();

    let result = part1(&grid);
    assert_eq!(209, result);
    println!("{}", result);

    let result = part2(&grid);
    assert_eq!(1492, result);
    println!("{}", result);
}

fn part1(grid: &Vec<&str>) -> usize {
    let mut activated = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cube) in row.chars().enumerate() {
            if cube == ACTIVE {
                activated.insert((i as i32, j as i32, 0));
            }
        }
    }

    let mut neighbors_cords = Vec::with_capacity(26); // 3^3-1
    for i in CORDS {
        for j in CORDS {
            for k in CORDS {
                if (i, j, k) != (0, 0, 0) {
                    neighbors_cords.push((i, j, k))
                }
            }
        }
    }

    for _ in CYCLES {
        let all: HashSet<_> = HashSet::from_iter(
            activated
                .iter()
                .map(|(i, j, k)| {
                    neighbors_cords
                        .iter()
                        .map(|(ic, jc, kc)| (*i + ic, *j + jc, *k + kc))
                        .collect::<Vec<_>>()
                })
                .flatten(),
        );

        let mut next = HashSet::new();
        for (i, j, k) in all.iter() {
            let neighbors_cnt = neighbors_cords
                .iter()
                .filter(|(ic, jc, kc)| activated.contains(&(i + ic, j + jc, k + kc)))
                .count();

            let cord = (*i, *j, *k);
            if (neighbors_cnt == 2 || neighbors_cnt == 3) && activated.contains(&cord) {
                next.insert(cord);
            } else if neighbors_cnt == 3 && !activated.contains(&cord) {
                next.insert(cord);
            }
        }
        activated = next;
    }

    activated.len()
}

fn part2(grid: &Vec<&str>) -> usize {
    let mut activated = HashSet::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cube) in row.chars().enumerate() {
            if cube == ACTIVE {
                activated.insert((i as i32, j as i32, 0, 0));
            }
        }
    }

    let mut neighbors_cords = Vec::with_capacity(80); // 3^4-1
    for i in CORDS {
        for j in CORDS {
            for k in CORDS {
                for l in CORDS {
                    if (i, j, k, l) != (0, 0, 0, 0) {
                        neighbors_cords.push((i, j, k, l))
                    }
                }
            }
        }
    }

    for _ in CYCLES {
        let all: HashSet<_> = HashSet::from_iter(
            activated
                .iter()
                .map(|(i, j, k, l)| {
                    neighbors_cords
                        .iter()
                        .map(|(ic, jc, kc, lc)| (*i + ic, *j + jc, *k + kc, *l + lc))
                        .collect::<Vec<_>>()
                })
                .flatten(),
        );

        let mut next = HashSet::new();
        for (i, j, k, l) in all.iter() {
            let neighbors_cnt = neighbors_cords
                .iter()
                .filter(|(ic, jc, kc, lc)| activated.contains(&(i + ic, j + jc, k + kc, l + lc)))
                .count();

            let cord = (*i, *j, *k, *l);
            if (neighbors_cnt == 2 || neighbors_cnt == 3) && activated.contains(&cord) {
                next.insert(cord);
            } else if neighbors_cnt == 3 && !activated.contains(&cord) {
                next.insert(cord);
            }
        }
        activated = next;
    }

    activated.len()
}
