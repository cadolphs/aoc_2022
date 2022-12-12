use std::{cmp::Reverse, collections::HashSet};

use itertools::Itertools;
use priority_queue::PriorityQueue;

pub fn run_day_12(input: String) {
    let (terrain, start, stop) = read_terrain(&input);

    let ans = find_path_length(&terrain, start, stop).unwrap();

    println!("It takes {} steps to reach the end", ans);

    let ans = find_best_start(&terrain, stop);

    println!("But from the best start it only takes {} steps to reach the end", ans);
}

fn find_best_start(terrain: &Vec<Vec<i8>>, stop: Vec2D) -> i32 {
    let starts = get_all_possible_starts(terrain);

    starts.into_iter().map(|start| find_path_length(terrain, start, stop))
    .filter(|length| length.is_some())
    .map(|x| x.unwrap())
    .min().unwrap()

}

fn find_path_length(terrain: &Vec<Vec<i8>>, start: Vec2D, stop: Vec2D) -> Option<i32> {
    let mut steps_from_start: PriorityQueue<Vec2D, Reverse<i32>> = PriorityQueue::new();
    let mut visited: HashSet<Vec2D> = HashSet::new();

    steps_from_start.push(start, Reverse(0));
    visited.insert(start);

    let rows = terrain.len();
    let cols = terrain[0].len();

    while !steps_from_start.is_empty() {
        let (current_node, Reverse(current_dist)) = steps_from_start.pop().unwrap(); // node with currently shortest paths
        
        if current_node == stop {
            return Some(current_dist);
        }

        let current_height = get_height(terrain, &current_node);
    
        for neighbor in get_adjacent_nodes(&current_node, rows, cols) {
            if get_height(terrain, &neighbor) <= current_height + 1 {
                if visited.insert(neighbor) { // returns true if new in set
                    steps_from_start.push(neighbor, Reverse(current_dist + 1));
                }
            }
        }
    }
    None
}

fn get_height(terrain: &Vec<Vec<i8>>, pos: &Vec2D) -> i8 {
    terrain[pos.0 as usize][pos.1 as usize]
}

fn get_adjacent_nodes(node: &Vec2D, rows: usize, cols: usize) -> Vec<Vec2D> {
    let check = |new_node: &Vec2D| {
        !(new_node.0 < 0
            || new_node.1 < 0
            || new_node.0 >= rows as i32
            || new_node.1 >= cols as i32)
    };

    [
        (node.0 + 1, node.1),
        (node.0 - 1, node.1),
        (node.0, node.1 + 1),
        (node.0, node.1 - 1),
    ]
    .into_iter()
    .map(|(x, y)| Vec2D(x, y))
    .filter(check)
    .collect_vec()
}

fn read_terrain(input: &str) -> (Vec<Vec<i8>>, Vec2D, Vec2D) {
    let mut terrain = Vec::new();

    let mut start_pos = Vec2D(0, 0);
    let mut end_pos = Vec2D(0, 0);

    for (row, line) in input.lines().enumerate() {
        terrain.push(vec![]);
        for (col, letter) in line.chars().enumerate() {
            terrain[row].push(letter_to_height(letter));
            if letter == 'S' {
                start_pos = Vec2D(row as i32, col as i32);
            } else if letter == 'E' {
                end_pos = Vec2D(row as i32, col as i32);
            }
        }
    }

    (terrain, start_pos, end_pos)
}

fn letter_to_height(letter: char) -> i8 {
    match letter {
        'S' => 1,
        'E' => 26,
        _ => {
            let code = letter as i8;
            if code >= ('a' as i8) && code <= ('z' as i8) {
                code - ('a' as i8) + 1
            } else {
                panic!("Invalid input.")
            }
        }
    }
}

fn get_all_possible_starts(terrain: &Vec<Vec<i8>>) -> Vec<Vec2D> {
    let mut starts = vec![];
    for (i, row) in terrain.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if *height == 1 {
                starts.push(Vec2D(i as i32, j as i32));
            }
        }
    }
    starts
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2D(i32, i32);

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_read_map() {
        let map = indoc!(
            "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi"
        );

        let (terrain, start, stop) = read_terrain(map);
        assert_eq!(terrain.len(), 5);
        assert_eq!(terrain[0].len(), 8);

        assert_eq!(start, Vec2D(0, 0));
        assert_eq!(stop, Vec2D(2, 5));

        assert_eq!(terrain[start.0 as usize][start.1 as usize], 1);
        assert_eq!(terrain[stop.0 as usize][stop.1 as usize], 26);
    }
}
