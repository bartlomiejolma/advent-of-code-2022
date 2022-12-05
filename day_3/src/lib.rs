use std::collections::{hash_map::RandomState, HashSet};

fn split_rucksacks(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn common_items(rucksacks: (&str, &str)) -> char {
    let first: HashSet<char, RandomState> = HashSet::from_iter(rucksacks.0.chars());
    let second = HashSet::from_iter(rucksacks.1.chars());

    first.intersection(&second).last().unwrap().clone()
}

fn score(letter: char) -> u32 {
    if (letter.is_lowercase()) {
        letter as u32 - 96
    } else {
        letter as u32 - 38
    }
}

fn line_score(line: &str) -> u32 {
    score(common_items(split_rucksacks(line)))
}
pub fn total_score(text: &str) -> u32 {
    text.lines()
        .map(str::trim)
        .map(line_score)
        .reduce(u32::wrapping_add)
        .unwrap()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_split_rucksacks() {
        // Given
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";

        let expected_split = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");

        // When
        let actual_split = split_rucksacks(line);

        // Then
        assert_eq!(actual_split, expected_split)
    }

    #[test]
    fn test_common_items() {
        // Given

        let rucksacks = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");

        let expected_common = 'p';
        // When
        let actual_common = common_items(rucksacks);

        // Then
        assert_eq!(actual_common, expected_common)
    }
    #[test]
    fn test_common_items_2() {
        // Given

        let rucksacks = ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL");

        let expected_common = 'L';
        // When
        let actual_common = common_items(rucksacks);

        // Then
        assert_eq!(actual_common, expected_common)
    }

    #[test]
    fn test_common_items_3() {
        // Given

        let rucksacks = ("PmmdzqPrV", "vPwwTWBwg");

        let expected_common = 'P';
        // When
        let actual_common = common_items(rucksacks);

        // Then
        assert_eq!(actual_common, expected_common)
    }

    #[test]
    fn test_score_a() {
        // Given

        let letter = 'a';

        let expected_score = 1;
        // When
        let actual_score = score(letter);

        // Then
        assert_eq!(expected_score, actual_score)
    }

    #[test]
    fn test_score_a_cap() {
        // Given

        let letter = 'A';

        let expected_score = 27;
        // When
        let actual_score = score(letter);

        // Then
        assert_eq!(expected_score, actual_score)
    }
    #[test]
    fn test_line_score() {
        // Given

        let line = "CrZsJsPPZsGzwwsLwLmpwMDw";

        let expected_score = 19;
        // When
        let actual_score = line_score(line);

        // Then
        assert_eq!(expected_score, actual_score)
    }

    #[test]
    fn test_total_score() {
        // Given

        let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let expected_score = 157;
        // When
        let actual_score = total_score(lines);

        // Then
        assert_eq!(expected_score, actual_score)
    }
}
