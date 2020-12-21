use advent2020::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

fn main() {
    let data = read_string("inputs/21.full");
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/21.test");
    assert_eq!(part1(&data), 5);
}

fn part1(input: &str) -> usize {
    let recipes = parse_recipes(&input);
    let knowns = assign_allergens(&recipes);
    recipes
        .iter()
        .flat_map(|r| &r.ingredients)
        .filter(|y| !knowns.contains_key(*y))
        .count()
}

#[test]
fn part2_small() {
    let data = read_string("inputs/21.test");
    assert_eq!(part2(&data), "mxmxvkd,sqjhc,fvjkl");
}

fn part2(input: &str) -> String {
    let recipes = parse_recipes(&input);
    let knowns = assign_allergens(&recipes);
    let mut bad_ingredients: Vec<&str> = knowns.keys().cloned().collect();
    bad_ingredients.sort_unstable_by_key(|y| knowns[y]);
    bad_ingredients.join(",")
}

fn assign_allergens<'a>(recipes: &'a [Recipe]) -> HashMap<&'a str, &'a str> {
    let mut assignments = HashMap::<&str, HashSet<&str>>::new();
    for r in recipes.iter() {
        for x in r.allergens.iter() {
            let candidates = assignments.entry(*x).or_insert_with(HashSet::new);
            if candidates.is_empty() {
                *candidates = r.ingredients.clone();
            } else {
                candidates.retain(|y| r.ingredients.contains(y));
            }
        }
    }
    let num_allergens = assignments.len();
    let mut known_ingredients = HashMap::<&str, &str>::new();
    while known_ingredients.len() < num_allergens {
        for (allergen, candidates) in assignments.iter_mut() {
            if candidates.len() == 1 {
                known_ingredients
                    .insert(candidates.iter().next().unwrap(), *allergen);
                candidates.clear();
            }
        }
        for candidates in assignments.values_mut() {
            candidates.retain(|y| !known_ingredients.contains_key(y));
        }
    }
    known_ingredients
}

#[derive(Debug)]
struct Recipe<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

fn parse_recipes(input: &str) -> Vec<Recipe> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^([a-z ]+) [(]contains ([a-z, ]+)[)]$").unwrap();
    }
    let mut recipes = vec![];
    for line in input.lines() {
        if let Some(captures) = RE.captures(line) {
            let ingredients =
                captures.get(1).unwrap().as_str().split(' ').collect();
            let allergens =
                captures.get(2).unwrap().as_str().split(", ").collect();
            recipes.push(Recipe {
                ingredients,
                allergens,
            });
        }
    }
    recipes
}
