mod days{
    pub mod day3;
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::days::day3 as today;
    use today::{part1, part2};
    use std::fs::read_to_string;

    #[test]
    fn part1_short() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..".to_string();
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part1_long(){
        let input = read_to_string("src/inputs/day2.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_short(){
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_long(){
        let input = read_to_string("src/inputs/day2.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
