#[derive(PartialEq, Debug, Clone)]
enum Mapping {
    Rock,
    Paper,
    Scissors,
}

fn score_from_your_selection(mapping: &Mapping) -> u32 {
    match mapping {
        Mapping::Rock => 1,
        Mapping::Paper => 2,
        Mapping::Scissors => 3,
    }
}

fn parse_line(line: &str) -> (Mapping, Mapping) {
    let first = match line.chars().nth(0).unwrap() {
        'A' => Mapping::Rock,
        'B' => Mapping::Paper,
        'C' => Mapping::Scissors,
        _ => panic!("Wrong selection"),
    };
    let second = match line.chars().nth(2).unwrap() {
        'X' => Mapping::Rock,
        'Y' => Mapping::Paper,
        'Z' => Mapping::Scissors,
        _ => panic!("Wrong selection"),
    };
    (first, second)
}

fn parse_line_2(line: &str) -> (Mapping, Outcome) {
    let first = match line.chars().nth(0).unwrap() {
        'A' => Mapping::Rock,
        'B' => Mapping::Paper,
        'C' => Mapping::Scissors,
        _ => panic!("Wrong selection"),
    };
    let second = match line.chars().nth(2).unwrap() {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Wrong selection"),
    };
    (first, second)
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}
fn score_from_result(result: &Outcome) -> u32 {
    match result {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    }
}
fn score_from_combination(selections: &(Mapping, Mapping)) -> u32 {
    let outcome = match selections {
        (x, y) if x == y => Outcome::Draw,
        (Mapping::Rock, Mapping::Scissors)
        | (Mapping::Paper, Mapping::Rock)
        | (Mapping::Scissors, Mapping::Paper) => Outcome::Lose,
        _ => Outcome::Win,
    };
    score_from_result(&outcome)
}
pub fn score(line: &str) -> u32 {
    let game = parse_line(line);
    println!("{:?}", game);
    score_from_combination(&game) + score_from_your_selection(&game.1)
}

fn mapping_from_score(game: (Mapping, Outcome)) -> (Mapping, Mapping) {
    match game {
        (Mapping::Paper, Outcome::Win) => (Mapping::Paper, Mapping::Scissors),
        (Mapping::Paper, Outcome::Lose) => (Mapping::Paper, Mapping::Rock),
        (Mapping::Rock, Outcome::Win) => (Mapping::Rock, Mapping::Paper),
        (Mapping::Rock, Outcome::Lose) => (Mapping::Rock, Mapping::Scissors),
        (Mapping::Scissors, Outcome::Win) => (Mapping::Scissors, Mapping::Rock),
        (Mapping::Scissors, Outcome::Lose) => (Mapping::Scissors, Mapping::Paper),
        (x, Outcome::Draw) => (x.clone(), x),
    }
}

pub fn score_2(line: &str) -> u32 {
    let game = parse_line_2(line);
    let mapping = mapping_from_score(game);
    score_from_combination(&mapping) + score_from_your_selection(&mapping.1)
}
fn total_score_generic(lines: &str, score_fn: fn(&str) -> u32) -> u32 {
    lines
        .lines()
        .map(score_fn)
        .reduce(u32::wrapping_add)
        .unwrap()
}

pub fn total_score(lines: &str) -> u32 {
    total_score_generic(lines, score)
}

pub fn total_score_2(lines: &str) -> u32 {
    total_score_generic(lines, score_2)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn sample_score_ay() {
        // Given
        let line = "A Y";
        let expected_score = 8;

        // When
        let actual_score = score(line);

        // Then
        assert_eq!(expected_score, actual_score)
    }

    #[test]

    fn sample_score_bx() {
        // Given
        let line = "B X";
        let expected_score = 1;

        // When
        let actual_score = score(line);

        // Then
        assert_eq!(expected_score, actual_score)
    }

    #[test]

    fn sample_score_cz() {
        // Given
        let line = "C Z";
        let expected_score = 6;

        // When
        let actual_score = score(line);

        // Then
        assert_eq!(expected_score, actual_score)
    }
    #[test]
    fn test_total() {
        // Given
        let lines = "A Y
B X
C Z";
        let expected_score = 15;

        // When
        let actual_score = total_score(lines);

        // Then
        assert_eq!(expected_score, actual_score)
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]

    fn sample_score_ay() {
        // Given
        let line = "A Y";
        let expected_score = 4;

        // When
        let actual_score = score_2(line);

        // Then
        assert_eq!(expected_score, actual_score)
    }

    #[test]

    fn sample_score_bx() {
        // Given
        let line = "B X";
        let expected_score = 1;

        // When
        let actual_score = score_2(line);

        // Then
        assert_eq!(expected_score, actual_score)
    }

    #[test]

    fn sample_score_cz() {
        // Given
        let line = "C Z";
        let expected_score = 7;

        // When
        let actual_score = score_2(line);

        // Then
        assert_eq!(expected_score, actual_score)
    }
    #[test]
    fn test_total() {
        // Given
        let lines = "A Y
B X
C Z";
        let expected_score = 12;

        // When
        let actual_score = total_score_2(lines);

        // Then
        assert_eq!(expected_score, actual_score)
    }
}
