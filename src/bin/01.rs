pub fn part_one(input: &str) -> Option<u32> {
    input.split("\n\n").map(get_elf_total_calories).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves_calories = input
        .split("\n\n")
        .map(get_elf_total_calories)
        .collect::<Vec<u32>>();
    elves_calories.sort();
    Some(elves_calories.into_iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn get_elf_total_calories(elf: &str) -> u32 {
    elf.lines()
        .map(|food| food.parse::<u32>().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
