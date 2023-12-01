use std::cmp::max;
use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Can't read input file");

    let tree_grid = build_tree_grid(&data);

    let visible_tree_count = part_one(&tree_grid);
    let most_beautiful = part_two(&tree_grid);

    println!("Part one: {}", visible_tree_count);
    println!("Part two: {}", most_beautiful);
}

fn part_one(tree_grid: &Vec<Vec<u32>>) -> usize {
    let width = tree_grid[0].len();
    let height = tree_grid.len();

    let edge_tree_count = height * 2 + width * 2 - 4;
    let mut visible_trees = HashSet::new();

    for y in 1..height - 1 {
        // check visible from left
        let mut highest_tree = tree_grid[y][0];
        for x in 1..tree_grid[y].len() - 1 {
            if tree_grid[y][x] > highest_tree {
                visible_trees.insert((x, y));
                highest_tree = tree_grid[y][x];
            }
        }

        //check visible from right
        highest_tree = tree_grid[y][width - 1];
        for x in (1..tree_grid[y].len() - 1).rev() {
            if tree_grid[y][x] > highest_tree {
                visible_trees.insert((x, y));
                highest_tree = tree_grid[y][x];
            }
        }
    }

    for x in 1..width - 1 {
        // check visible from top
        let mut highest_tree = tree_grid[0][x];
        for y in 1..height - 1 {
            if tree_grid[y][x] > highest_tree {
                visible_trees.insert((x, y));
                highest_tree = tree_grid[y][x];
            }
        }

        //check visible from bottom
        highest_tree = tree_grid[height - 1][x];
        for y in (1..height - 1).rev() {
            if tree_grid[y][x] > highest_tree {
                visible_trees.insert((x, y));
                highest_tree = tree_grid[y][x];
            }
        }
    }

    visible_trees.len() + edge_tree_count
}

fn part_two(tree_grid: &Vec<Vec<u32>>) -> u32 {
    let mut most_beautiful = 0;

    for y in 0..tree_grid.len() {
        for x in 0..tree_grid[y].len() {
            most_beautiful = max(most_beautiful, calculate_vista(&tree_grid, x, y));
        }
    }

    most_beautiful
}

fn calculate_vista(tree_grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let width = tree_grid[0].len();
    let height = tree_grid.len();

    // look to the right
    let mut visible_tree_count_right = 0;
    for xt in x + 1..width {
        visible_tree_count_right += 1;
        if tree_grid[y][x] <= tree_grid[y][xt] {
            break;
        }
    }

    // look to the left
    let mut visible_tree_count_left = 0;
    for xt in (0..x).rev() {
        visible_tree_count_left += 1;
        if tree_grid[y][x] <= tree_grid[y][xt] {
            break;
        }
    }

    // look down
    let mut visible_tree_count_down = 0;
    for yt in y + 1..height {
        visible_tree_count_down += 1;
        if tree_grid[y][x] <= tree_grid[yt][x] {
            break;
        }
    }

    // look up
    let mut visible_tree_count_up = 0;
    for yt in (0..y).rev() {
        visible_tree_count_up += 1;
        if tree_grid[y][x] <= tree_grid[yt][x] {
            break;
        }
    }

    visible_tree_count_right
        * visible_tree_count_left
        * visible_tree_count_up
        * visible_tree_count_down
}

fn build_tree_grid(data: &str) -> Vec<Vec<u32>> {
    let mut tree_grid: Vec<Vec<u32>> = Vec::new();

    for line in data.lines() {
        tree_grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    tree_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_vista() {
        let data = fs::read_to_string("input-small.txt").expect("Can't read input file");
        let tree_grid = build_tree_grid(&data);

        assert_eq!(4, calculate_vista(&tree_grid, 2, 1));
        assert_eq!(8, calculate_vista(&tree_grid, 2, 3));
    }
}
