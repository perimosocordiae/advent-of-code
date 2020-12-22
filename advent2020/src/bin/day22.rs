use advent2020::*;
use std::collections::VecDeque;

fn main() {
    let data = read_string("inputs/22.full");
    println!("Part 1: {}", part1(&data));
    // println!("Part 2: {}", part2(&data));
}

#[test]
fn part1_small() {
    let data = read_string("inputs/22.test");
    assert_eq!(part1(&data), 306);
}

fn part1(input: &str) -> usize {
    let (mut deck1, mut deck2) = parse_decks(&input);
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    score_deck(deck1.make_contiguous()) + score_deck(deck2.make_contiguous())
}

fn score_deck(deck: &[usize]) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x * (i + 1))
        .sum()
}

fn parse_decks(data: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut chunks = data.split("\n\n");
    let deck1 = parse_deck(chunks.next().unwrap());
    let deck2 = parse_deck(chunks.next().unwrap());
    (deck1, deck2)
}

fn parse_deck(chunk: &str) -> VecDeque<usize> {
    chunk.lines().skip(1).map(|x| x.parse().unwrap()).collect()
}
