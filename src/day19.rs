use std::collections::HashMap;
use std::fs::read_to_string;

const A: &str = "\"a\"";
const B: &str = "\"b\"";
const OR: &str = "|";
const START_RULE: &str = "0";

pub fn run() {
    let input = read_to_string("src/day19-input.txt").unwrap();

    let result = count_matches(input.split("\n\n").collect());
    assert_eq!(144, result);
    println!("{}", result);

    let input = input
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");
    let result = count_matches(input.split("\n\n").collect());
    assert_eq!(260, result);
    println!("{}", result);
}

fn count_matches(input: Vec<&str>) -> usize {
    let rules: HashMap<_, _> = input[0]
        .lines()
        .map(|x| {
            let s: Vec<_> = x.split(":").collect();
            let value: Vec<_> = s[1].trim().split(" ").collect();
            (s[0], value)
        })
        .collect();
    let messages: Vec<_> = input[1].lines().collect();

    let mut cache = HashMap::new();
    messages
        .iter()
        .filter(|&x| is_valid(&rules, START_RULE, x, 0, x.len(), &mut cache))
        .count()
}

fn is_valid<'a>(
    rules: &HashMap<&str, Vec<&'a str>>,
    rule_key: &'a str,
    msg: &'a str,
    start: usize,
    end: usize,
    cache: &mut HashMap<(&'a str, &'a str, usize, usize), bool>,
) -> bool {
    if let Some(v) = cache.get(&(rule_key, msg, start, end)) {
        return *v;
    }

    if rule_key == A || rule_key == B {
        return msg[start..end] == rule_key[1..=1];
    }

    let rule = rules.get(rule_key).unwrap();
    let (l, r): (Vec<&str>, Vec<&str>) = match rule.iter().position(|&x| x == OR) {
        Some(v) => (rule[..v].to_vec(), rule[v + 1..].to_vec()),
        None => (rule.clone(), vec![]),
    };

    let r = is_valid_subrule(rules, msg, start, end, l, cache)
        || is_valid_subrule(rules, msg, start, end, r, cache);
    cache.insert((rule_key, msg, start, end), r);
    r
}

fn is_valid_subrule<'a>(
    rules: &HashMap<&str, Vec<&'a str>>,
    msg: &'a str,
    start: usize,
    end: usize,
    subrules: Vec<&'a str>,
    cache: &mut HashMap<(&'a str, &'a str, usize, usize), bool>,
) -> bool {
    if start == end && subrules.len() == 0 {
        return true;
    }
    if start == end || subrules.len() == 0 {
        return false;
    }

    for i in start..end {
        if is_valid(rules, subrules[0], msg, start, i + 1, cache)
            && is_valid_subrule(rules, msg, i + 1, end, subrules[1..].to_vec(), cache)
        {
            return true;
        }
    }

    false
}
