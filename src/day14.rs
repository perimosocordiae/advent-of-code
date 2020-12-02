use std::fmt::Write;

pub fn run() {
    let (num_recipes, pattern) = setup("503761");
    println!("Part 1: {}", part1(num_recipes));
    println!("Part 2: {}", part2(&pattern));
}

fn setup(input: &str) -> (usize, Vec<usize>) {
    let num_recipes = input.parse().unwrap();
    let pattern = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    (num_recipes, pattern)
}

fn part1(num_recipes: usize) -> String {
    let mut scoreboard = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    loop {
        let score1 = scoreboard[elf1];
        let score2 = scoreboard[elf2];
        let sum = score1 + score2;
        if sum >= 10 {
            scoreboard.push(sum / 10);
        }
        scoreboard.push(sum % 10);
        let n = scoreboard.len();
        if n >= num_recipes + 10 {
            let mut result = String::new();
            for x in scoreboard[num_recipes..num_recipes + 10].iter() {
                write!(result, "{}", x).unwrap();
            }
            return result;
        }
        elf1 += score1 + 1;
        elf2 += score2 + 1;
        elf1 %= n;
        elf2 %= n;
    }
}

fn part2(pattern: &[usize]) -> usize {
    let mut scoreboard = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    loop {
        let score1 = scoreboard[elf1];
        let score2 = scoreboard[elf2];
        let sum = score1 + score2;
        if sum >= 10 {
            scoreboard.push(sum / 10);
            if let Some(ans) = check_match(&scoreboard, &pattern) {
                return ans;
            }
        }
        scoreboard.push(sum % 10);
        if let Some(ans) = check_match(&scoreboard, &pattern) {
            return ans;
        }
        let n = scoreboard.len();
        elf1 += score1 + 1;
        elf2 += score2 + 1;
        elf1 %= n;
        elf2 %= n;
    }
}

fn check_match(scoreboard: &[usize], pattern: &[usize]) -> Option<usize> {
    let n = scoreboard.len();
    let m = pattern.len();
    if n < m {
        return None;
    }
    for idx in (0..m).rev() {
        if scoreboard[n - m + idx] != pattern[idx] {
            return None;
        }
    }
    Some(n - m)
}
