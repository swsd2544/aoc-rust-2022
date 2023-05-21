pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let elf = decode_elfcode(iter.next().unwrap());
                let human = decode_humancode(iter.next().unwrap());
                let adder = match human {
                    Code::Rock => 1,
                    Code::Paper => 2,
                    Code::Scissors => 3,
                };

                adder
                    + match human {
                        Code::Rock => match elf {
                            Code::Scissors => MatchOutcome::Win,
                            Code::Rock => MatchOutcome::Draw,
                            Code::Paper => MatchOutcome::Lose,
                        },
                        Code::Paper => match elf {
                            Code::Rock => MatchOutcome::Win,
                            Code::Paper => MatchOutcome::Draw,
                            Code::Scissors => MatchOutcome::Lose,
                        },
                        Code::Scissors => match elf {
                            Code::Paper => MatchOutcome::Win,
                            Code::Scissors => MatchOutcome::Draw,
                            Code::Rock => MatchOutcome::Lose,
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
                let human = decode_human_win(iter.next().unwrap());
                human.value()
                    + match elf {
                        Code::Rock => match human {
                            MatchOutcome::Win => 2,
                            MatchOutcome::Lose => 3,
                            MatchOutcome::Draw => 1,
                        },
                        Code::Paper => match human {
                            MatchOutcome::Win => 3,
                            MatchOutcome::Lose => 1,
                            MatchOutcome::Draw => 2,
                        },
                        Code::Scissors => match human {
                            MatchOutcome::Win => 1,
                            MatchOutcome::Lose => 2,
                            MatchOutcome::Draw => 3,
                        },
                    }
            })
            .sum(),
    )
}

enum Code {
    Rock,
    Paper,
    Scissors,
}

enum MatchOutcome {
    Win,
    Lose,
    Draw,
}

impl MatchOutcome {
    fn value(&self) -> u32 {
        match self {
            MatchOutcome::Win => 6,
            MatchOutcome::Lose => 0,
            MatchOutcome::Draw => 3,
        }
    }
}

fn decode_elfcode(input: &str) -> Code {
    match input {
        "A" => Code::Rock,
        "B" => Code::Paper,
        "C" => Code::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn decode_humancode(input: &str) -> Code {
    match input {
        "X" => Code::Rock,
        "Y" => Code::Paper,
        "Z" => Code::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn decode_human_win(input: &str) -> MatchOutcome {
    match input {
        "X" => MatchOutcome::Lose,
        "Y" => MatchOutcome::Draw,
        "Z" => MatchOutcome::Win,
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
