use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("day8/input.txt").expect("Can't read input file");

    let mut tree_grid: Vec<Vec<u32>> = Vec::new();

    for line in data.lines() {
        tree_grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

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

    let visible_tree_count = visible_trees.len() + edge_tree_count;

    println!("{}", visible_tree_count);
}
