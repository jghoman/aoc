use std::cmp;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_line(line: &str) -> u32 {
    let results: Vec<&str> = line
        .split(':')
        .last()
        .expect("Should have got the results")
        .split(';')
        .map(|s| s.split(','))
        .flatten()
        .map(str::trim)
        .collect();

    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    for r in results {
        let mut stats = r.split(' ');
        let count = stats
            .next()
            .expect("Should be a count")
            .parse::<u32>()
            .expect("That we can parse");
        let color = stats.next().expect("Should be a color");

        match color {
            "red" => min_red = cmp::max(min_red, count),
            "green" => min_green = cmp::max(min_green, count),
            "blue" => min_blue = cmp::max(min_blue, count),
            _ => panic!("Should have had a color!"),
        }
    }

    min_red * min_blue * min_green
}

fn part2(input: &str) -> String {
    let result = input.lines().map(parse_line).sum::<u32>();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();

        assert_eq!("2286", part2(&input));
    }
}
