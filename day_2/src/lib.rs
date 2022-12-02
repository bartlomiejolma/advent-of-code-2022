#[derive(PartialEq, Debug)]
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

pub fn total_score(lines: &str) -> u32 {
    lines.lines().map(score).reduce(u32::wrapping_add).unwrap()
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
