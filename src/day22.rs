use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Debug)]
enum Winner {
    P1,
    P2,
}

impl Winner {
    fn score(&self, p1: &VecDeque<u32>, p2: &VecDeque<u32>) -> u32 {
        let winner_cards = match self {
            Winner::P1 => p1,
            Winner::P2 => p2,
        };
        let (result, _) = winner_cards
            .iter()
            .rev()
            .fold((0, 1), |(result, multiplier), x| {
                (result + x * multiplier, multiplier + 1)
            });
        result
    }
}

pub fn run() {
    let input = read_to_string("src/day22-input.txt").unwrap();
    let players: Vec<VecDeque<u32>> = input
        .split("\n\n")
        .map(|x| x.lines().skip(1).map(|y| y.parse().unwrap()).collect())
        .collect();
    assert_eq!(2, players.len());

    let (mut p1, mut p2) = (players[0].clone(), players[1].clone());
    let winner = combat(&mut p1, &mut p2);
    let result = winner.score(&p1, &p2);
    println!("{}", result);

    let (mut p1, mut p2) = (players[0].clone(), players[1].clone());
    let winner = recursive_combat(&mut p1, &mut p2, &mut HashSet::new(), &mut HashSet::new());
    let result = winner.score(&p1, &p2);
    assert_eq!(34594, result);
    println!("{}", result);
}

fn combat(p1: &mut VecDeque<u32>, p2: &mut VecDeque<u32>) -> Winner {
    loop {
        if p1.len() == 0 {
            return Winner::P2;
        }
        if p2.len() == 0 {
            return Winner::P1;
        }

        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        match p1_card.cmp(&p2_card) {
            Ordering::Less => {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
            Ordering::Greater => {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            }
            Ordering::Equal => unreachable!(),
        };
    }
}

fn recursive_combat(
    p1: &mut VecDeque<u32>,
    p2: &mut VecDeque<u32>,
    prevs_p1: &mut HashSet<VecDeque<u32>>,
    prevs_p2: &mut HashSet<VecDeque<u32>>,
) -> Winner {
    if p1.len() == 0 {
        return Winner::P2;
    }
    if p2.len() == 0 {
        return Winner::P1;
    }
    if prevs_p1.contains(p1) && prevs_p2.contains(p2) {
        return Winner::P1;
    }

    prevs_p1.insert(p1.clone());
    prevs_p2.insert(p2.clone());

    let p1_card = p1.pop_front().unwrap();
    let p2_card = p2.pop_front().unwrap();
    let winner = if p1.len() as u32 >= p1_card && p2.len() as u32 >= p2_card {
        let mut new_p1 = p1.clone();
        new_p1.truncate(p1_card as usize);
        let mut new_p2 = p2.clone();
        new_p2.truncate(p2_card as usize);
        recursive_combat(
            &mut new_p1,
            &mut new_p2,
            &mut HashSet::new(),
            &mut HashSet::new(),
        )
    } else {
        match p1_card.cmp(&p2_card) {
            Ordering::Less => Winner::P2,
            Ordering::Greater => Winner::P1,
            Ordering::Equal => unreachable!(),
        }
    };

    match winner {
        Winner::P1 => {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        }
        Winner::P2 => {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    };

    recursive_combat(p1, p2, prevs_p1, prevs_p2)
}
