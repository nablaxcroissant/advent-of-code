#[derive(Clone, Copy)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

enum Game{
    Win(Action),
    Loss(Action),
    Draw(Action),
}


pub fn part_1(input: &str) -> i32{
    input
        .trim()
        .lines()
        .map(|line| line.split(" ")
            .map(|str| check_action(str).unwrap())
            .collect::<Vec<Action>>())
        .map(|actions| count_game(check_win(&actions).unwrap()))
        .sum()
}

pub fn part_2(input: &str) -> i32{
    input
        .trim()
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|round| count_round(&round).unwrap())
        .sum()
}

fn count_round(actions: &[&str]) -> Option<i32>{
     match &actions[..] {
        [elf, game] => {
            Some(count_game(correct_move(check_win_2(game, check_action(elf).unwrap()).unwrap())))
        },
        _ => None

    }
}

fn correct_move(outcome: Game) -> Game{
    match outcome{
        Game::Win(opp) => match opp{
            Action::Rock => Game::Win(Action::Paper),
            Action::Paper => Game::Win(Action::Scissors),
            Action::Scissors => Game::Win(Action::Rock),
        } 
        Game::Loss(opp) => match opp{
            Action::Rock => Game::Loss(Action::Scissors),
            Action::Paper => Game::Loss(Action::Rock),
            Action::Scissors => Game::Loss(Action::Paper),
        }
        Game::Draw(opp) => Game::Draw(opp),
    }
}

fn check_win_2(key: &str, opp: Action) -> Option<Game>{
    match key{
        "X" => Some(Game::Loss(opp)),
        "Y" => Some(Game::Draw(opp)),
        "Z" => Some(Game::Win(opp)),
        _ => None,
    }
}

fn check_action(action: &str) -> Option<Action>{
    match action{
        "A" => Some(Action::Rock),
        "B" => Some(Action::Paper),
        "C" => Some(Action::Scissors),
        "X" => Some(Action::Rock),
        "Y" => Some(Action::Paper),
        "Z" => Some(Action::Scissors),
        _ => None,
    }
}

fn check_win(actions: &[Action]) -> Option<Game>{
    match &actions[..] {
        [elf, you] => {
            match elf{
                Action::Rock => {
                    match you{
                        Action::Rock => Some(Game::Draw(*you)),
                        Action::Paper => Some(Game::Win(*you)),
                        Action::Scissors => Some(Game::Loss(*you))
                    }
                }
                Action::Paper => {
                    match you{
                        Action::Rock => Some(Game::Loss(*you)),
                        Action::Paper => Some(Game::Draw(*you)),
                        Action::Scissors => Some(Game::Win(*you))
                    }
                }
                Action::Scissors => {
                    match you{
                        Action::Rock => Some(Game::Win(*you)),
                        Action::Paper => Some(Game::Loss(*you)),
                        Action::Scissors => Some(Game::Draw(*you)),
                    }
                }
            }   
        } 
        _ => None
    }
}

fn count_game(game: Game) -> i32{
    match game{
        Game::Win(action) => 6 + count_action(action),
        Game::Loss(action) => count_action(action),
        Game::Draw(action) => 3 + count_action(action),
    }
}

fn count_action(action: Action) -> i32{
    match action{
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_1(input), 8392);
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("input.txt");
        assert_eq!(part_2(input), 10116);
    }


}
