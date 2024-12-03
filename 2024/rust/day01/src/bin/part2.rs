use std::collections::HashMap;


fn main() {
    let input = include_str!("../../input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut split = line.split_whitespace();

    (split.next().expect("First tuple").parse::<i32>().unwrap(), split.next().expect("Second tuple").parse::<i32>().unwrap())
}

fn part2(input: &str) -> String {
    let mut left = Vec::new();
    let mut right = Vec::new();

    input
        .lines()
        .map(parse_line)
        .for_each(|(a,b)| { 
            left.push(a); 
            right.push(b);
        });

    right.sort();

    let mut grouped = HashMap::new();
    for contig in right.chunk_by(|x, y| *x == *y).into_iter() {
        grouped.insert(contig[0], contig.len() as i32);

    }
    
    left.iter()
        .map(|x| x * *grouped.get(x).get_or_insert(&0))
        .sum::<i32>()
        .to_string()

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
        3   3".to_string();

        assert_eq!("31", part2(&input));

    }
}
