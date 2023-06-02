pub fn part_1(input: &str) -> i32 {
    input
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum())
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> i32 {
    let mut vec = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum())
        .collect::<Vec<i32>>();
    vec.sort_by(|a,b| b.cmp(a));
    (0..3).fold(0, |acc, i| {
        acc + vec[i]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(include_str!("input.txt")), 420);
    }

    #[test]
    fn part_2_test(){
        assert_eq!(part_2(include_str!("input.txt")), 69);
    }
}
