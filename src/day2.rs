use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let mut collection = HashMap::new();
        for c in line.chars() {
            let count = collection.entry(c).or_insert(0); 
            *count += 1;
        }

        if collection.values().find(|&&x| x == 2).is_some() { twos += 1; }
        if collection.values().find(|&&x| x == 3).is_some() { threes += 1; }
    }
    twos * threes
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    for (word1, word2) in iproduct!(input.lines(), input.lines()) {
        if let Ok(dist) = hamming_distance(word1, word2) {
            if dist != 1 {
                continue; 
            }
            return word1.chars()
                        .zip(word2.chars())
                        .filter(|(c1, c2)| c1 == c2)
                        .map(|(c1, c2)| c1)
                        .collect();
        }
    }
    "Not found".to_string()
}

fn hamming_distance(x: &str, y: &str) -> Result<usize, &'static str> {
    if(x.len() != y.len()) {
        return Err("Hamming distance can only be performed on equal length strings");
    }

    Ok(x.chars()
        .zip(y.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), "fgij");
    }
}

/*
Day2 - Part1/(default)  time:   [292.68 us 294.12 us 295.69 us]                                   
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

Day2 - Part2/(default)  time:   [3.6265 ms 3.6415 ms 3.6588 ms]                                    
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
*/
