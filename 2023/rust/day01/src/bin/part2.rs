fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_line(orig: &str) -> u32 {
    let mut digits:Vec<u32> = Vec::new();

    let mut i = 0;

    while i < orig.chars().count() {
        // Maybe num
        let c = orig.chars().nth(i).expect("Should be a character");
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).expect("Should be parseable"));

        } else {
            // Check if start of word
            let substr = &orig[i..];
            // There are definitely better ways to do this, but I'm getting tired
            match true {
                _ if substr.starts_with("one") => digits.push(1),
                _ if substr.starts_with("two") => digits.push(2),
                _ if substr.starts_with("three") => digits.push(3),
                _ if substr.starts_with("four") => digits.push(4),
                _ if substr.starts_with("five") => digits.push(5),
                _ if substr.starts_with("six") => digits.push(6),
                _ if substr.starts_with("seven") => digits.push(7),
                _ if substr.starts_with("eight") => digits.push(8),
                _ if substr.starts_with("nine") => digits.push(9),
                _ => (), // Do nothing if no match
            }
        }

        i = i + 1;
    }


    let first = digits.first().expect("Should have gotten a number");
    let last = digits.last().expect("Should have gotten a second number");

    return first * 10 + last;
}

fn part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|l| parse_line(l))
        .sum::<u32>();
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();
        assert_eq!("142", part1(&input));
    }

    #[test]
    fn it_works_part2() {
        let input = "two1nine
 eightwothree
 abcone2threexyz
 xtwone3four
 4nineeightseven2
 zoneight234
 7pqrstsixteen"
            .to_string();
        assert_eq!("281", part1(&input));
    }

    #[test]
    fn argh() {
        let input = "eightwothree".to_string();

        assert_eq!("83", part1(&input));

    }
}
