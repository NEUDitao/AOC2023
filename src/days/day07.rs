use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn score_hand(hand: &str) -> HandType {
    let cards = hand.chars().collect::<Vec<_>>();
    let cards_as_set = BTreeSet::from_iter(cards.clone());
    if cards_as_set.len() == 1 {
        return HandType::FiveOfAKind;
    } else if cards_as_set.len() == 2 {
        let four_of_a_kind = cards.iter().any(|card| {
            cards
                .iter()
                .filter(|c| &card == c)
                .collect::<Vec<_>>()
                .len()
                == 4
        });

        if four_of_a_kind {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    } else if cards_as_set.len() == 3 {
        let three_of_a_kind = cards.iter().any(|card| {
            cards
                .iter()
                .filter(|c| &card == c)
                .collect::<Vec<_>>()
                .len()
                == 3
        });
        if three_of_a_kind {
            return HandType::ThreeOfAKind;
        } else {
            return HandType::TwoPair;
        }
    } else if cards_as_set.len() == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn score_hand_with_joker(hand: &str) -> HandType {
    let cards = hand.chars().collect::<Vec<_>>();
    if cards.contains(&'J') {
        if BTreeSet::from_iter(cards).len() == 1 {
            return HandType::FiveOfAKind;
        }

        let cards = BTreeSet::from_iter(hand.chars());
        let not_joker_cards = cards
            .clone()
            .into_iter()
            .filter(|card| card != &'J')
            .collect::<Vec<_>>();
        not_joker_cards
            .into_iter()
            .map(|card| {
                let new_hand = hand.replace("J", &card.to_string());
                score_hand(&new_hand)
            })
            .max()
            .unwrap()
    } else {
        score_hand(hand)
    }
}

fn score_card(card: char) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        a => panic!("{}", a),
    }
}

fn score_card_with_joker(card: char) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        a => panic!("{}", a),
    }
}

fn break_tie(hand_a: &str, hand_b: &str) -> Ordering {
    let binding = hand_a.split(" ").collect_vec();
    let hand_a = binding.get(0).unwrap();
    let binding = hand_b.split(" ").collect_vec();
    let hand_b = binding.get(0).unwrap();

    let zip = hand_a.chars().zip(hand_b.chars());

    for (card_a, card_b) in zip {
        if score_card(card_a) > score_card(card_b) {
            return Ordering::Greater;
        } else if score_card(card_a) < score_card(card_b) {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

fn break_tie_with_joker(hand_a: &str, hand_b: &str) -> Ordering {
    let binding = hand_a.split(" ").collect_vec();
    let hand_a = binding.get(0).unwrap();
    let binding = hand_b.split(" ").collect_vec();
    let hand_b = binding.get(0).unwrap();

    let zip = hand_a.chars().zip(hand_b.chars());

    for (card_a, card_b) in zip {
        if score_card_with_joker(card_a) > score_card_with_joker(card_b) {
            return Ordering::Greater;
        } else if score_card_with_joker(card_a) < score_card_with_joker(card_b) {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

fn get_bid(hand: &str) -> usize {
    let split = hand.split(" ").collect_vec();
    split.get(1).unwrap().parse().unwrap()
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07").unwrap();
    let rows = input.split('\n').collect::<Vec<_>>();

    let sorted_hands = rows
        .clone()
        .into_iter()
        .map(|hand| {
            let mut split = hand.split_whitespace();
            let res = score_hand(split.next().unwrap());
            (res, hand.to_owned())
        })
        .collect::<Vec<_>>();

    let mut map = BTreeMap::new();
    for (key, value) in sorted_hands {
        map.entry(key).or_insert(Vec::new()).push(value.clone());
    }

    let rank = map
        .into_iter()
        .flat_map(|(_, hand)| {
            let mut new_hand = hand.clone();
            new_hand.sort_by(|a, b| break_tie(a, b));
            new_hand
        })
        .collect::<Vec<_>>();

    let sol1: usize = rank
        .into_iter()
        .enumerate()
        .map(|(i, hand)| get_bid(&hand) * (i + 1))
        .sum();

    let sorted_hands = rows
        .clone()
        .into_iter()
        .map(|hand| {
            let mut split = hand.split_whitespace();
            let res = score_hand_with_joker(split.next().unwrap());
            (res, hand.to_owned())
        })
        .collect::<Vec<_>>();

    let mut map = BTreeMap::new();
    for (key, value) in sorted_hands {
        map.entry(key).or_insert(Vec::new()).push(value.clone());
    }

    let rank = map
        .into_iter()
        .flat_map(|(_, hand)| {
            let mut new_hand = hand.clone();
            new_hand.sort_by(|a, b| break_tie_with_joker(a, b));
            new_hand
        })
        .collect::<Vec<_>>();

    let sol2: usize = rank
        .into_iter()
        .enumerate()
        .map(|(i, hand)| get_bid(&hand) * (i + 1))
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}
