mod days{
    pub mod day1;
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::days::day1 as today;
    use today::{part1, part2};
    use std::fs::read_to_string;

    #[test]
    fn part1_short() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet".to_string();
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn part1_long(){
        let input = read_to_string("src/inputs/day1.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 54708);
    }

    #[test]
    fn part2_short(){
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen".to_string();
        let result = part2(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn part2_long(){
        let input = read_to_string("src/inputs/day1.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
