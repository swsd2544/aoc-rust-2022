use std::collections::{HashSet, btree_map::Range};

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let n = lines.len();

    // build non visible trees hashset
    let mut non_visible_trees = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            non_visible_trees.insert((i, j));
        }
    }

    // delete edges
    for i in 0..n {
        non_visible_trees.remove(&(i, 0));
        non_visible_trees.remove(&(0, i));
        non_visible_trees.remove(&(i, n-1));
        non_visible_trees.remove(&(n-1, i));
    }

    // delete from left
    for i in 1..n-1 {
        let mut max = lines[i][0];
        for j in 1..n-1 {
            let height = lines[i][j];
            if height > max {
                max = height;
                non_visible_trees.remove(&(i, j));
            }
        }
    }

    // delete from right
    for i in 1..n-1 {
        let mut max = lines[i][n-1];
        for j in (1..n-1).rev() {
            let height = lines[i][j];
            if height > max {
                max = height;
                non_visible_trees.remove(&(i, j));
            }
        }
    }

    // delete from top
    for j in 1..n-1 {
        let mut max = lines[0][j];
        for i in 1..n-1 {
            let height = lines[i][j];
            if height > max {
                max = height;
                non_visible_trees.remove(&(i, j));
            }
        }
    }

    // delete from bottom
    for j in 1..n-1 {
        let mut max = lines[n-1][j];
        for i in (1..n-1).rev() {
            let height = lines[i][j];
            if height > max {
                max = height;
                non_visible_trees.remove(&(i, j));
            }
        }
    }

    let visible_trees_count = n*n - non_visible_trees.len();
    Some(visible_trees_count.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let n = lines.len();

    let mut highest_scenic_score = 0;
    for x in 1..n-1 {
        for y in 1..n-1 {
            let mut scenic_score = 1;

            let mut count = 1;
            for j in (1..y).rev() {
                if lines[x][j] >= lines[x][y] {
                    break;
                }
                count += 1;
            }
            scenic_score *= count;

            let mut count = 1;
            for j in y+1..n-1 {
                if lines[x][j] >= lines[x][y] {
                    break;
                }
                count += 1;
            }
            scenic_score *= count;

            let mut count = 1;
            for i in (1..x).rev() {
                if lines[i][y] >= lines[x][y] {
                    break;
                }
                count += 1;
            }
            scenic_score *= count;

            let mut count = 1;
            for i in x+1..n-1 {
                if lines[i][y] >= lines[x][y] {
                    break;
                }
                count += 1;
            }
            scenic_score *= count;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }
       
    Some(highest_scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
