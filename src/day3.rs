use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

lazy_static! {
    // Input example: #1 @ 1,3: 4x4
    static ref SQUARE: Regex = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").expect("Failed to make regex");
}

#[derive(Debug)]
pub struct Square {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Square> {
    input.lines()
         .map(|line|  {
                let caps = SQUARE.captures(line).expect("Fail to capture");
                Square { 
                    id: caps[1].parse().expect("Failed to parse id"),
                    x:  caps[2].parse().expect("Failed to parse x"),
                    y:  caps[3].parse().expect("Failed to parse y"),
                    width: caps[4].parse().expect("Failed to parse width"),
                    height: caps[5].parse().expect("Failed to parse height")
                }
            })
         .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Square]) -> usize {
    let mut grid: HashMap<(usize, usize), u32> = HashMap::new();
    for square in input.iter() {
        let x = square.x;
        let y = square.y;
        for width_index in (0..square.width) {
            for height_index in (0..square.height) {
                *grid.entry((x+width_index, y+height_index)).or_default() += 1;
            }
        }
    }

    grid.values().filter(|&&x| x > 1).count()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Square]) -> usize {
    // Create the grid of cuts setting each index with the square's ID that occupied it
    // If another square has the same cut, remove both squares from the untouched set
    let mut untouched = HashSet::new();
    let mut grid = HashMap::new();
    for square in input.iter() {
        let x = square.x;
        let y = square.y;
        untouched.insert(square.id);
        for width_index in (0..square.width) {
            for height_index in (0..square.height) {
                let curr_square = grid.entry((x+width_index, y+height_index)).or_insert(square.id);
                if *curr_square != square.id {
                    untouched.remove(curr_square);
                    untouched.remove(&square.id);
                }
            }
        }
    }

    if untouched.len() > 1 {
        panic!("untouched contains more than one untouched square");
    }

    for i in untouched.drain() { return i; }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&input_generator("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2")), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&input_generator("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2")), 3);
    }
}
