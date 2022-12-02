#[derive(Debug, Clone, Copy)]
enum Symbol {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Copy, Clone)]
enum State {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn theirs(c: char) -> Symbol {
    match c {
        'A' => Symbol::Rock,
        'B' => Symbol::Paper,
        'C' => Symbol::Scissors,

        _ => panic!(),
    }
}

fn mine(c: char) -> Symbol {
    match c {
        'X' => Symbol::Rock,
        'Y' => Symbol::Paper,
        'Z' => Symbol::Scissors,

        _ => panic!(),
    }
}

fn intention(c: char) -> State {
    match c {
        'X' => State::Lose,
        'Y' => State::Draw,
        'Z' => State::Win,

        _ => panic!(),
    }
}

#[derive(Debug)]
struct Round {
    theirs: Symbol,
    mine: Symbol,
}

impl Round {
    fn result(&self) -> State {
        match (self.mine, self.theirs) {
            (Symbol::Rock, Symbol::Paper) => State::Lose,
            (Symbol::Rock, Symbol::Scissors) => State::Win,

            (Symbol::Paper, Symbol::Scissors) => State::Lose,
            (Symbol::Paper, Symbol::Rock) => State::Win,

            (Symbol::Scissors, Symbol::Rock) => State::Lose,
            (Symbol::Scissors, Symbol::Paper) => State::Win,

            _ => State::Draw,
        }
    }
}
impl std::str::FromStr for Round {
    type Err = std::string::ParseError;

    #[allow(clippy::iter_nth_zero)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.chars().nth(0).unwrap();
        let y = s.chars().nth(2).unwrap();

        Ok(Round {
            theirs: theirs(x),
            mine: mine(y),
        })
    }
}
#[derive(Debug)]
struct Intent {
    symbol: Symbol,
    goal: State,
}

impl std::str::FromStr for Intent {
    type Err = std::string::ParseError;

    #[allow(clippy::iter_nth_zero)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.chars().nth(0).unwrap();
        let y = s.chars().nth(2).unwrap();

        Ok(Intent {
            symbol: theirs(x),
            goal: intention(y),
        })
    }
}
impl Intent {
    fn result(&self) -> Symbol {
        match (self.symbol, self.goal) {
            (Symbol::Rock, State::Lose) => Symbol::Scissors,
            (Symbol::Rock, State::Win) => Symbol::Paper,

            (Symbol::Paper, State::Lose) => Symbol::Rock,
            (Symbol::Paper, State::Win) => Symbol::Scissors,

            (Symbol::Scissors, State::Lose) => Symbol::Paper,
            (Symbol::Scissors, State::Win) => Symbol::Rock,

            (.., State::Draw) => self.symbol,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score = 0;
    for inp in input.lines() {
        let round = inp.parse::<Round>().unwrap();

        total_score += round.mine as u32;
        total_score += round.result() as u32;
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score = 0;
    for inp in input.lines() {
        let round = inp.parse::<Intent>().unwrap();

        total_score += round.goal as u32;
        total_score += round.result() as u32;
    }

    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
