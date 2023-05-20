pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let elf = decode_elfcode(iter.next().unwrap());
                let human = decode_humancode(iter.next().unwrap());
                let adder = match human {
                    Code::ROCK => 1,
                    Code::PAPER => 2,
                    Code::SCISSORS => 3,
                };

                adder
                    + match human {
                        Code::ROCK => match elf {
                            Code::SCISSORS => MatchOutcome::WIN,
                            Code::ROCK => MatchOutcome::DRAW,
                            Code::PAPER => MatchOutcome::LOSE,
                        },
                        Code::PAPER => match elf {
                            Code::ROCK => MatchOutcome::WIN,
                            Code::PAPER => MatchOutcome::DRAW,
                            Code::SCISSORS => MatchOutcome::LOSE,
                        },
                        Code::SCISSORS => match elf {
                            Code::PAPER => MatchOutcome::WIN,
                            Code::SCISSORS => MatchOutcome::DRAW,
                            Code::ROCK => MatchOutcome::LOSE,
                        },
                    }
                    .value()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let elf = decode_elfcode(iter.next().unwrap());
                let human = decode_humanwin(iter.next().unwrap());
                human.value()
                    + match elf {
                        Code::ROCK => match human {
                            MatchOutcome::WIN => 2,
                            MatchOutcome::LOSE => 3,
                            MatchOutcome::DRAW => 1,
                        },
                        Code::PAPER => match human {
                            MatchOutcome::WIN => 3,
                            MatchOutcome::LOSE => 1,
                            MatchOutcome::DRAW => 2,
                        },
                        Code::SCISSORS => match human {
                            MatchOutcome::WIN => 1,
                            MatchOutcome::LOSE => 2,
                            MatchOutcome::DRAW => 3,
                        },
                    }
            })
            .sum(),
    )
}

enum Code {
    ROCK,
    PAPER,
    SCISSORS,
}

enum MatchOutcome {
    WIN,
    LOSE,
    DRAW,
}

impl MatchOutcome {
    fn value(&self) -> u32 {
        match self {
            MatchOutcome::WIN => 6,
            MatchOutcome::LOSE => 0,
            MatchOutcome::DRAW => 3,
        }
    }
}

fn decode_elfcode(input: &str) -> Code {
    match input {
        "A" => Code::ROCK,
        "B" => Code::PAPER,
        "C" => Code::SCISSORS,
        _ => panic!("Invalid input"),
    }
}

fn decode_humancode(input: &str) -> Code {
    match input {
        "X" => Code::ROCK,
        "Y" => Code::PAPER,
        "Z" => Code::SCISSORS,
        _ => panic!("Invalid input"),
    }
}

fn decode_humanwin(input: &str) -> MatchOutcome {
    match input {
        "X" => MatchOutcome::LOSE,
        "Y" => MatchOutcome::DRAW,
        "Z" => MatchOutcome::WIN,
        _ => panic!("Invalid input"),
    }
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
