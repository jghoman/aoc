use std::collections::{BTreeMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, space1, u32},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");

    let output = part1(input);
    dbg!(output);

    let output2 = part2(input);
    dbg!(output2);
}

#[derive(Debug)]
struct Scratchcard {
    card_num: u32,
    winning_numbers: Vec<u32>,
    held_numbers: Vec<u32>,
}

#[derive(Clone, Debug)]
struct CardAndCount {
    card_num: u32,
    count: u32,
}

fn scratchcard(input: &str) -> IResult<&str, Scratchcard> {
    let (input, _) = terminated(tag("Card "), multispace0)(input)?;
    let (input, id) = (u32)(input)?;
    let (input, _) = terminated(tag(": "), multispace0)(input)?;
    let (input, winning_numbers) = separated_list1(space1, u32)(input)?;
    let (input, _) = terminated(tag(" | "), multispace0)(input)?;
    let (input, held_numbers) = separated_list1(space1, u32)(input)?;

    Ok((
        input,
        Scratchcard {
            card_num: id,
            winning_numbers: winning_numbers,
            held_numbers: held_numbers,
        },
    ))
}

fn parse_scratchcards(input: &str) -> IResult<&str, Vec<Scratchcard>> {
    let (input, scratchcards) = separated_list1(line_ending, scratchcard)(input)?;

    Ok((input, scratchcards))
}

fn num_matches(s: Scratchcard) -> Vec<u32> {
    // Could just build the hashsets initially, but want to be ready for whatever part 2 wants
    let held: HashSet<u32> = s.held_numbers.iter().cloned().collect();
    let winning: HashSet<u32> = s.winning_numbers.iter().cloned().collect();

    held.intersection(&winning).cloned().collect()
}

fn part1(input: &str) -> String {
    let (_, scratchcards) = parse_scratchcards(input).expect("Should parse");

    scratchcards
        .into_iter()
        .map(num_matches)
        //.inspect(|p| println!("{:?}", p))
        .map(|p| p.len() as u32)
        .filter(|&p| p > 0)
        .map(|p| 2_i32.pow(p - 1) as u32)
        .sum::<u32>()
        .to_string()
}

fn matches_with_card_num(s: Scratchcard) -> (u32, Vec<u32>) {
    let held: HashSet<u32> = s.held_numbers.iter().cloned().collect();
    let winning: HashSet<u32> = s.winning_numbers.iter().cloned().collect();

    (s.card_num, held.intersection(&winning).cloned().collect())
}

fn copies_from_cards(
    cards: &Vec<CardAndCount>,
    mymap: &BTreeMap<u32, CardAndCount>,
) -> Vec<CardAndCount> {
    let mut new_cards: Vec<CardAndCount> = Vec::new();

    for card in cards {
        let num = card.card_num;
        let count = card.count;
        let start = num + 1;
        let end = start + count;

        for i in start..end {
            let value = mymap.get(&i).expect("Should be there").clone();
            new_cards.push(value);
        }
    }

    new_cards
}

fn part2(input: &str) -> String {
    let (_, scratchcards) = parse_scratchcards(input).expect("Should parse");

    let card_count_map: BTreeMap<u32, CardAndCount> = scratchcards
        .into_iter()
        .map(matches_with_card_num)
        //.inspect(|p| println!("{:?}", p))
        .map(|p| {
            (
                p.0,
                CardAndCount {
                    card_num: p.0,
                    count: p.1.len() as u32,
                },
            )
        })
        .collect();

    let mut count = 0_u32;

    let mut result = card_count_map.values().cloned().collect::<Vec<_>>();

    loop {
        count += result.len() as u32;
        let new_results = copies_from_cards(&result, &card_count_map);

        if new_results.len() == 0 {
            break;
        }
        result = new_results;
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        assert_eq!("13", part1(&input))
    }

    #[test]
    fn part2_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        assert_eq!("30", part2(&input))
    }
}
