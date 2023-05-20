use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut priority_sum = 0;
    for line in input.lines() {
        let mut hs = HashSet::<char>::new();
        let line = line.trim();
        let (a, b) = line.split_at(line.len() / 2);
        for c in b.chars() {
            if !a.contains(c) || !c.is_ascii() || hs.contains(&c) {
                continue;
            }
            hs.insert(c);
            if c as u32 > 'a' as u32 {
                priority_sum += (c as u32 - 'a' as u32) + 1;
            } else {
                priority_sum += (c as u32 - 'A' as u32) + 27;
            }
        }
    }
    Some(priority_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut priority_sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let line = lines[i];
        let mut hs = HashSet::<char>::new();
        for c in line.trim().chars() {
            if !lines[i + 1].contains(c)
                || !lines[i + 2].contains(c)
                || !c.is_ascii()
                || hs.contains(&c)
            {
                continue;
            }
            hs.insert(c);
            if c as u32 > 'a' as u32 {
                priority_sum += (c as u32 - 'a' as u32) + 1;
            } else {
                priority_sum += (c as u32 - 'A' as u32) + 27;
            }
        }
    }
    Some(priority_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
