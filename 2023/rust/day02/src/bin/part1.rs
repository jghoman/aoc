fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {

    return "hi".to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected = "hi".to_string();

        assert_eq!(expected, part1("whatever"));
    }    
}