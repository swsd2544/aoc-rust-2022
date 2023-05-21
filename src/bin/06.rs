use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    find_unique_substring(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_unique_substring(input, 14)
}

fn find_unique_substring(input: &str, n: u32) -> Option<u32> {
    let mut hs: HashSet<char> = HashSet::new();
    let chars = input.chars().collect::<Vec<char>>();
    let mut index = n;
    'outer: for window in chars.windows(n as usize) {
        hs.clear();
        for c in window.iter() {
            
            if hs.contains(c) {
                index += 1;
                continue 'outer;
            }
            hs.insert(*c);
        }
        break;
    }
    Some(index)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
