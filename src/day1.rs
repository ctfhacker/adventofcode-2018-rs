#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(|n| n.parse::<i32>().unwrap()).sum()
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
}
