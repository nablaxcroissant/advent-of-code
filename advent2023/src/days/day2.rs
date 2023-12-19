#![allow(dead_code)]
use regex::Regex;

#[derive(Debug)]
enum Color{
    Green,
    Red,
    Blue,
}

pub fn part1(input: String) -> u32 {
    let id_re = Regex::new(r"Game (\d+)").unwrap();
    let pull_re = Regex::new(r"(\d+) (green|red|blue)").unwrap();
    input.split("\n")
        .filter(|line| {line.len()>0})
        .map(|line| {
            let id = id_re.captures(line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
            let pulls: Vec<(u32, Color)> = pull_re.captures_iter(line)
                .map(|c| {
                    let n = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let color = match c.get(2).unwrap().as_str(){
                        "green" => Some(Color::Green),
                        "blue" => Some(Color::Blue),
                        "red" => Some(Color::Red),
                        _ => None,
                    }.unwrap();
                    (n, color)
                })
                .collect();
            //print!("{:?}", pulls);
            if pulls.into_iter()
                .all(|(n, color)|{
                    n <= match color{
                        Color::Red => 12,
                        Color::Green => 13,
                        Color::Blue => 14,
                    }
                }) {
                    
                    id
                } else {0}
        })
        .sum()
}

pub fn part2(input: String) -> u32 {
    let pull_re = Regex::new(r"(\d+) (green|red|blue)").unwrap();
    input.split("\n")
        .filter(|line| {line.len()>0})
        .map(|line| {
            let mut greens = Vec::new();
            let mut reds = Vec::new();
            let mut blues = Vec::new();
            pull_re.captures_iter(line)
                .for_each(|c| {
                    let n = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    match c.get(2).unwrap().as_str(){
                        "green" => greens.push(n),
                        "blue" => blues.push(n),
                        "red" => reds.push(n),
                        _ => (),
                    };
                });
            //print!("{:?}", pulls);
            greens.sort_by(|a,b| b.cmp(&a));
            reds.sort_by(|a,b| b.cmp(&a));
            blues.sort_by(|a,b| b.cmp(&a));
            let result = greens[0]*reds[0]*blues[0];
            println!("{:?}", result);
            result
        })
        .sum()
}