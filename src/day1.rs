use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(|n| n.parse::<i32>().unwrap()).sum()
}

#[aoc(day1, part2, set)]
pub fn part2_set(input: &str) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut freq: i32 = 0;
    seen.insert(freq);
    for line in input.lines().cycle() {
        let curr_num = line.parse::<i32>().unwrap(); 
        freq += curr_num;
        if(seen.contains(&freq)) {
            return freq;
        }
        seen.insert(freq);
    }
    panic!("Shouldn't reach here");
}

#[aoc(day1, part2, vec)]
pub fn part2_vec(input: &str) -> i32 {
    let mut seen: Vec<i32> = Vec::new();
    let mut freq: i32 = 0;
    seen.push(0);
    for line in input.lines().cycle() {
        let curr_num = line.parse::<i32>().unwrap(); 
        freq += curr_num;
        if(seen.contains(&freq)) {
            return freq;
        }
        seen.push(freq);
    }
    panic!("Shouldn't reach here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1("+1\n+1\n+1"), 3);
        assert_eq!(part1("+1\n+1\n-2"), 0);
        assert_eq!(part1("-1\n-2\n-3"), -6);
    }

    #[test]
    fn test2_set() {
        assert_eq!(part2_set("+1\n-1"), 0);
        assert_eq!(part2_set("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(part2_set("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2_set("+7\n+7\n-2\n-7\n-4"), 14);
    }

    #[test]
    fn test2_list() {
        assert_eq!(part2_list("+1\n-1"), 0);
        assert_eq!(part2_list("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(part2_list("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2_list("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
