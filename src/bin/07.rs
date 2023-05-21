use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut cwd = String::with_capacity(70);
    let mut map: HashMap<String, u32> = HashMap::new();
    for line in input.lines().filter(|&l| l != "$ ls" && &l[0..3] != "dir") {
        match line {
            "$ cd /"  => {
                cwd.push('.');
            },
            "$ cd .." => {
                let i = cwd.rfind("/").unwrap();
                cwd.truncate(i);
            },
            _ if line.starts_with("$ cd") => {
                cwd.push('/');
                cwd.push_str(&line[5..]);
            }
            _ => {
                let fsize: u32 = line.split_whitespace().next().unwrap().parse().unwrap();
                let dsize = map.entry(cwd.clone()).or_insert(0);
                *dsize += fsize;
                cwd.match_indices("/").for_each(|(i, _)| {
                    let dsize = map.entry(cwd[0..i].to_owned()).or_insert(0);
                    *dsize += fsize;
                })
            }
        }
    }
    
    Some(map.values().filter(|&dsize| *dsize <= 100_000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cwd = String::with_capacity(70);
    let mut map: HashMap<String, u32> = HashMap::new();
    for line in input.lines().filter(|&l| l != "$ ls" && &l[0..3] != "dir") {
        match line {
            "$ cd /"  => {
                cwd.push('.');
            },
            "$ cd .." => {
                let i = cwd.rfind("/").unwrap();
                cwd.truncate(i);
            },
            _ if line.starts_with("$ cd") => {
                cwd.push('/');
                cwd.push_str(&line[5..]);
            }
            _ => {
                let fsize: u32 = line.split_whitespace().next().unwrap().parse().unwrap();
                let dsize = map.entry(cwd.clone()).or_insert(0);
                *dsize += fsize;
                cwd.match_indices("/").for_each(|(i, _)| {
                    let dsize = map.entry(cwd[0..i].to_owned()).or_insert(0);
                    *dsize += fsize;
                })
            }
        }
    }

    let mut min_dsize: u32 = map["."];
    if min_dsize < 40_000_000 {
        return Some(0);
    }
    let constraint_dsize: u32 = min_dsize - 40_000_000;
    for &dsize in map.values() {
        if dsize < min_dsize && dsize >= constraint_dsize {
            min_dsize = dsize;
        }
    }
    Some(min_dsize)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
