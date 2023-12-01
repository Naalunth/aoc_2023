use aho_corasick::AhoCorasick;
use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .bytes()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c - b'0');
            let first_digit = digits.clone().next().unwrap();
            let last_digit = digits.last().unwrap();
            (first_digit * 10 + last_digit) as u64
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let first_digit = word_to_digit(first_match(line, &VALID_MATCHES));
            let last_digit = word_to_digit(last_match(line, &VALID_MATCHES));
            (first_digit * 10 + last_digit) as u64
        })
        .sum()
}

const VALID_MATCHES: [&str; 19] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn word_to_digit(word: &str) -> u8 {
    match word {
        "0" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Invalid word: {}", word),
    }
}

fn first_match<'a>(haystack: &str, words: &[&'a str]) -> &'a str {
    for idx in 0..haystack.bytes().len() {
        for &word in words {
            if haystack.as_bytes()[idx..].starts_with(word.as_bytes()) {
                return word;
            }
        }
    }
    panic!("No word found in: {}", haystack);
}

fn last_match<'a>(haystack: &str, words: &[&'a str]) -> &'a str {
    for idx in (0..haystack.bytes().len()).rev() {
        for &word in words {
            if haystack.as_bytes()[idx..].starts_with(word.as_bytes()) {
                return word;
            }
        }
    }
    panic!("No word found in: {}", haystack);
}

#[aoc(day1, part2, aho_corasick)]
pub fn part_2_aho_corasick(input: &str) -> u64 {
    let ac = AhoCorasick::new(VALID_MATCHES).unwrap();
    input.lines()
        .map(|line| {
            let first_digit = word_to_digit(&line[ac.find(line).unwrap().span()]);
            let last_digit = word_to_digit(&line[ac.find_overlapping_iter(line).last().unwrap().span()]);
            (first_digit * 10 + last_digit) as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part_1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            part_2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }
}
