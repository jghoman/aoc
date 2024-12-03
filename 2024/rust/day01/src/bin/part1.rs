use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let input = include_str!("../../input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut split = line.split_whitespace();

    (
        split.next().expect("First tuple").parse::<i32>().unwrap(),
        split.next().expect("Second tuple").parse::<i32>().unwrap(),
    )
}

fn part1(input: &str) -> String {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    input.lines().map(parse_line).for_each(|(a, b)| {
        left.push(Reverse(a));
        right.push(Reverse(b))
    });

    let mut result = 0;
    while !left.is_empty() {
        let Reverse(l) = left.pop().unwrap();
        let Reverse(r) = right.pop().unwrap();

        result += (l - r).abs();
    }

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3"
            .to_string();

        assert_eq!("11", part1(&input));
    }
}
