use std::collections::HashMap;
use std::fs::read_to_string;

const GOLD_BAG: &str = "shiny gold";

pub fn run() {
    let input = read_to_string("src/day7-input.txt").unwrap();

    let mut all_holds: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    for row in input.lines() {
        let mut s = row.split("contain");
        let bag = s.next().unwrap();
        let bag = &bag[..bag.len() - 6];

        all_holds.insert(String::from(bag), vec![]);
        let rest = s.collect::<String>();
        if rest == " no other bags." {
            continue;
        }

        let holds: Vec<_> = rest.split(",").collect();
        for holds_bag in holds {
            let mut c = holds_bag.trim_start().chars();
            let count: u32 = c.next().unwrap() as u32 - '0' as u32;
            let name: String = c.collect();
            let name = name.trim_start();
            let name = name.split("bag").next().unwrap().trim_end();

            all_holds
                .entry(String::from(bag))
                .or_insert(vec![])
                .push((String::from(name), count));
        }
    }

    let result = all_holds.keys().fold(0, |acc, x| {
        if x != GOLD_BAG && contains_gold(&all_holds, x) {
            acc + 1
        } else {
            acc
        }
    });
    assert_eq!(151, result);
    println!("{}", result);

    let result = count(&all_holds, &GOLD_BAG.to_owned());
    assert_eq!(41559, result);
    println!("{}", result);
}

fn contains_gold(holds: &HashMap<String, Vec<(String, u32)>>, bag_name: &String) -> bool {
    let v = holds.get(bag_name).unwrap();
    for (name, _) in v.iter() {
        if name == &GOLD_BAG.to_owned() || contains_gold(&holds, name) {
            return true;
        }
    }

    false
}

fn count(holds: &HashMap<String, Vec<(String, u32)>>, bag_name: &String) -> u32 {
    let v = holds.get(bag_name).unwrap();
    v.iter()
        .fold(0, |acc, (name, c)| acc + c + c * count(&holds, name))
}
