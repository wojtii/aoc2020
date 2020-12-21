use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}
pub fn run() {
    let input = read_to_string("src/day21-input.txt").unwrap();
    let mut all_ingredients = HashSet::new();
    let mut all_allergens = HashSet::new();
    let food_list: Vec<_> = input
        .lines()
        .map(|x| {
            let raw: Vec<_> = x.split(" (contains ").collect();
            let ingredients = raw[0]
                .split(" ")
                .map(|x| {
                    let i = String::from(x.trim());
                    all_ingredients.insert(i.clone());
                    i
                })
                .collect();
            let allergens = raw[1][..raw[1].len() - 1]
                .split(",")
                .map(|x| {
                    let a = String::from(x.trim());
                    all_allergens.insert(a.clone());
                    a
                })
                .collect();

            Food {
                ingredients,
                allergens,
            }
        })
        .collect();

    let mut all: HashMap<_, _> = all_ingredients
        .iter()
        .map(|x| (x, all_allergens.clone()))
        .collect();
    let mut count = HashMap::new();
    for food in food_list.iter() {
        for ingredient in food.ingredients.iter() {
            *count.entry(ingredient).or_insert(0) += 1;
        }

        for allergen in food.allergens.iter() {
            for ingredient in all_ingredients.iter() {
                if !food.ingredients.contains(ingredient) {
                    all.get_mut(ingredient).unwrap().remove(allergen);
                }
            }
        }
    }

    let result = all_ingredients.iter().fold(0, |acc, x| {
        if all.get(&x).unwrap().len() == 0 {
            acc + count.get(&x).unwrap()
        } else {
            acc
        }
    });
    assert_eq!(2162, result);
    println!("{}", result);

    let mut visited = HashSet::new();
    let mut dangerous = Vec::with_capacity(all_allergens.len());
    while dangerous.len() < all_allergens.len() {
        for ingredient in all_ingredients.iter() {
            let ings: Vec<_> = all
                .get(&ingredient)
                .unwrap()
                .iter()
                .filter(|&x| !visited.contains(x))
                .collect();
            if ings.len() != 1 {
                continue;
            }
            dangerous.push((ings[0].clone(), ingredient));
            visited.insert(ings[0]);
        }
    }

    dangerous.sort();
    let dangerous_ingredients: Vec<_> = dangerous.iter().map(|(_, i)| i.clone().clone()).collect();
    let result = dangerous_ingredients.join(",");
    assert_eq!("lmzg,cxk,bsqh,bdvmx,cpbzbx,drbm,cfnt,kqprv", result);
    println!("{}", result);
}
