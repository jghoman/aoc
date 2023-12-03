fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

static REDS: u32 = 12;
static GREENS: u32 = 13;
static BLUES: u32 = 14;

fn parse_line(line: &str) -> u32 {
    let game = line.split(':').next().expect("Should have got the game");
    let game_num = game
        .split(' ')
        .last()
        .expect("Should have got game num")
        .parse::<u32>()
        .expect("And should have parsed it.");

    let any_bad = line
        .split(':')
        .last()
        .expect("Should have got the results")
        .split(';')
        .map(|s| s.split(','))
        .flatten()
        .map(str::trim)
        // We don't need to split again for part1, but I expect we will for part2, so going ahead
        // and doing it now.
        .map(|s| {
            let mut stats = s.split(' ');
            let count = stats
                .next()
                .expect("Should be a count")
                .parse::<u32>()
                .expect("That we can parse");
            let color = stats.next().expect("Should be a color");

            match color {
                "green" if count > GREENS => return 0,
                "red" if count > REDS => return 0,
                "blue" if count > BLUES => return 0,
                _ => count,
            }
        })
        .any(|i| i == 0);

    if any_bad {
        0
    } else {
        game_num
    }
}

fn part1(input: &str) -> String {
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

        assert_eq!("8", part1(&input));
    }
}
