use std::collections::{hash_map::RandomState, HashSet};

fn parse_pair(pair: &str) -> (u32, u32) {
    let mut nums = pair.split('-');
    (
        nums.next().unwrap().parse::<u32>().unwrap(),
        nums.next().unwrap().parse::<u32>().unwrap(),
    )
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let mut pairs = line.split(',');
    (
        parse_pair(pairs.next().unwrap()),
        parse_pair(pairs.next().unwrap()),
    )
}

fn do_overlap(pairs: &((u32, u32), (u32, u32))) -> bool {
    let first: HashSet<u32, RandomState> = HashSet::from_iter(pairs.0 .0..=pairs.0 .1);
    let second: HashSet<u32, RandomState> = HashSet::from_iter(pairs.1 .0..=pairs.1 .1);
    // println!("{:?} {:?}", first, second);
    first.is_subset(&second) || first.is_superset(&second)
}

fn do_overlap_2(pairs: &((u32, u32), (u32, u32))) -> bool {
    let first: HashSet<u32, RandomState> = HashSet::from_iter(pairs.0 .0..=pairs.0 .1);
    let second: HashSet<u32, RandomState> = HashSet::from_iter(pairs.1 .0..=pairs.1 .1);
    // println!("{:?} {:?}", first, second);
    first.intersection(&second).count() > 0
}

fn count_pairs_gen(text: &str, overlap_fn: fn(&((u32, u32), (u32, u32))) -> bool) -> usize {
    text.lines()
        .map(str::trim)
        .map(parse_line)
        .filter(overlap_fn)
        .count()
}
pub fn count_pairs(text: &str) -> usize {
    count_pairs_gen(text, do_overlap)
}

pub fn count_pairs_2(text: &str) -> usize {
    count_pairs_gen(text, do_overlap_2)
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_pairs() {
        // Given
        let text = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        let expected_pairs = 2;

        // When
        let actual_pairs = count_pairs(text);

        // Then
        assert_eq!(actual_pairs, expected_pairs)
    }

    #[test]
    fn test_pairs_2() {
        // Given
        let text = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        let expected_pairs = 4;

        // When
        let actual_pairs = count_pairs_2(text);

        // Then
        assert_eq!(actual_pairs, expected_pairs)
    }
}
