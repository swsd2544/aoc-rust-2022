pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let mut splits = line
                    .splitn(4, &['-', ','])
                    .map(|s| s.parse::<u32>().unwrap());
                let start1 = splits.next().unwrap();
                let end1: u32 = splits.next().unwrap();
                let start2 = splits.next().unwrap();
                let end2 = splits.next().unwrap();
                (start1 >= start2 && end1 <= end2) || (start2 >= start1 && end2 <= end1)
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let mut splits = line
                    .splitn(4, &['-', ','])
                    .map(|s| s.parse::<u32>().unwrap());
                let start1 = splits.next().unwrap();
                let end1: u32 = splits.next().unwrap();
                let start2 = splits.next().unwrap();
                let end2 = splits.next().unwrap();
                (start1 <= start2 && end1 >= start2) || (start2 <= start1 && end2 >= start1)
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
