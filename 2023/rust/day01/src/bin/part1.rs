fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_line(l: &str) -> u32 {
    println!("Here is l: {}", l);
    let first = l
        .to_string()
        .chars()
        .find(|&c| char::is_numeric(c))
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last = l
        .to_string()
        .chars()
        .rev()
        .find(|&c| char::is_numeric(c))
        .unwrap()
        .to_digit(10)
        .unwrap();

    let val = first * 10 + last;
    println!("Here is the number {}", val);
    return first * 10 + last;
}

fn part1(input: &str) -> String {
    let result = input
        .split('\n')
        .into_iter()
        .map(|l| parse_line(l))
        .sum::<u32>();
    dbg!(result);
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
}
