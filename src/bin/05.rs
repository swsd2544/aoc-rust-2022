pub fn part_one(input: &str) -> Option<String> {
    let mut splits = input.splitn(2, "\n\n");
    let mut stacks = splits.next().unwrap().split('\n').rev();

    let len = stacks.next().unwrap().split_whitespace().count();
    let mut stacks = stacks.fold(vec![vec![]; len], |mut acc, line| {
        line
            .char_indices()
            .skip(1)
            .step_by(4)
            .filter(|(_, c)| {
                *c != ' '
            })
            .for_each(|(i, c)| {
                acc[i / 4].push(c)
            });
        acc
    });

    splits.next().unwrap().trim().split('\n').for_each(|line| {
        let moves = line.split_whitespace().collect::<Vec<&str>>();
        let num = moves[1].parse::<usize>().unwrap();
        let from = moves[3].parse::<usize>().unwrap() - 1;
        let to = moves[5].parse::<usize>().unwrap() - 1;
        for _ in 0..num {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    });

    let mut result = String::new();
    stacks.iter().for_each(|st| {
        match st.last() {
            Some(c) => result.push(*c),
            None => ()
        }
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut splits = input.splitn(2, "\n\n");
    let mut stacks = splits.next().unwrap().split('\n').rev();

    let len = stacks.next().unwrap().split_whitespace().count();
    let mut stacks = stacks.fold(vec![vec![]; len], |mut acc, line| {
        line
            .char_indices()
            .skip(1)
            .step_by(4)
            .filter(|(_, c)| {
                *c != ' '
            })
            .for_each(|(i, c)| {
                acc[i / 4].push(c)
            });
        acc
    });

    splits.next().unwrap().trim().split('\n').for_each(|line| {
        let moves = line.split_whitespace().collect::<Vec<&str>>();
        let num = moves[1].parse::<usize>().unwrap();
        let from = moves[3].parse::<usize>().unwrap() - 1;
        let to = moves[5].parse::<usize>().unwrap() - 1;
        let len = stacks[from].len();
        for i in 0..num {
            let ch = stacks[from][len - num + i];
            stacks[to].push(ch);
        }
        stacks[from].truncate(len - num);
    });

    let mut result = String::new();
    stacks.iter().for_each(|st| {
        match st.last() {
            Some(c) => result.push(*c),
            None => ()
        }
    });
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
