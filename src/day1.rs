use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(|n| n.parse::<i32>().unwrap()).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
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
    fn test2() {
        assert_eq!(part2("+1\n-1"), 0);
        assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
