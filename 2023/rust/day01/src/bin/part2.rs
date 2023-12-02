fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn words_to_digit(orig: &str) -> Vec<u32> {
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
            if substr.starts_with("one") {
                digits.push(1);
            }
            if substr.starts_with("two") {
                digits.push(2);
            }
            if substr.starts_with("three") {
                digits.push(3);
            }
            if substr.starts_with("four") {
                digits.push(4);
            }
            if substr.starts_with("five") {
                digits.push(5);
            }
            if substr.starts_with("six") {
                digits.push(6);
            }
            if substr.starts_with("seven") {
                digits.push(7);
            }
            if substr.starts_with("eight") {
                digits.push(8);
            }
            if substr.starts_with("nine") {
                digits.push(9);
            }
        }

        i = i + 1;
    }


    digits
}

fn parse_line(l: &str) -> u32 {
    let nums = words_to_digit(l);
    let first = nums.first().expect("Should have gotten a number");
    let last = nums.last().expect("Should have gotten a second number");

    return first * 10 + last;
}

fn part1(input: &str) -> String {
    let result = input
        .split('\n')
        .into_iter()
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
