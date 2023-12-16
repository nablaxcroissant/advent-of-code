#![allow(dead_code)]
use regex::Regex;

pub fn part1(input: String) -> u32 {
    input.split("\n")
        .filter(|l|{
            l.len() > 0
        })
        .map(|l|{
            let num_digits = l.chars()
                .filter(|c|{
                    c.is_numeric()
                })
                .map(|d| {
                    d.to_digit(10).unwrap()
                })
                .collect::<Vec<u32>>();
            let len = num_digits.len();
            num_digits[0]*10+num_digits[len-1]
        })
        .sum()
}

pub fn part2(input: String) -> u32 {
    let patterns = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", r"\d"];
    let re = patterns.iter()
        .map(|pattern| {Regex::new(&pattern).unwrap()})
        .collect::<Vec<Regex>>();
    input.split("\n")
        .filter(|line| {line.len() > 0})
        .map(|line| {
            let mut digits = Vec::new();
            re.iter()
                .for_each(|r| {
                    r.find_iter(line)
                        .for_each(|m| {
                            let result = match m.as_str() {
                                "one" => 1,
                                "two" => 2,
                                "three" => 3,
                                "four" => 4,
                                "five" => 5,
                                "six" => 6,
                                "seven" => 7,
                                "eight" => 8,
                                "nine" => 9,
                                _ => m.as_str().chars().map(|d| d.to_digit(10).unwrap()).next().unwrap(),
                            };
                            digits.push((m.start(), result));
                        })
                });
            digits.sort_by(|a,b| {a.0.cmp(&b.0)});
            let len = digits.len();
            digits[0].1*10+digits[len-1].1
        })
        .sum()
}