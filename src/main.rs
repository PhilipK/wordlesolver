use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let mut words = create_word_list();
    let stdin = io::stdin();
    let mut current_word: Option<String> = None;
    let mut current_colors: Option<String> = None;

    println!("Write your guess (5 letter limit, then hit enter)");
    println!("Try one of theese");
    for word in words.iter().take(5) {
        println!("{}", word);
    }
    println!("");

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if current_word == None {
            current_word = Some(line);
            println!("Write your colors (b=black,y=yellow,g=green) then hit enter");
            continue;
        }
        if current_colors == None {
            current_colors = Some(line);

            let word = current_word.unwrap();
            let input = current_colors.unwrap();

            let requirements = string_to_requirements(input, word).unwrap();
            words.retain(|word| word_matches_requirements(word, &requirements));

            sort_list_by_score(&mut words);
            current_colors = None;
            current_word = None;
            println!("{} possible words remaining", words.len());
            println!("Try one of theese");
            for word in words.iter().take(5) {
                println!("{}", word);
            }
            println!("");

            println!("Write your guess (5 letter limit, then hit enter)");
        }
    }
}

fn sort_list_by_score(words: &mut Vec<String>) {
    let all_words = words.clone();
    let mut score_map = HashMap::new();
    for current_word in &all_words {
        let mut word_score = 0;
        for target in &all_words {
            word_score += calculate_score(current_word, target);
        }
        score_map.insert(current_word, word_score);
    }
    words.sort_by_cached_key(|k| score_map.get(k));
    words.reverse();
}

fn string_to_requirements<N: AsRef<str>>(input: N, word: N) -> Result<[Requirement; 5], String> {
    let input = input.as_ref();
    let word = word.as_ref();

    if input.len() != 5 {
        return Err("Input is wrong length".to_string());
    }
    Ok([
        to_requirement(word, 0, &input[0..1]),
        to_requirement(word, 1, &input[1..2]),
        to_requirement(word, 2, &input[2..3]),
        to_requirement(word, 3, &input[3..4]),
        to_requirement(word, 4, &input[4..5]),
    ])
}

fn to_requirement(word: &str, index: usize, char: &str) -> Requirement {
    let s = word[index..index + 1].to_string();
    match char {
        "g" => Requirement::Green(s),
        "y" => Requirement::Yellow(s),
        "b" => Requirement::Black(s),
        _ => panic!("unknown color"),
    }
}

enum Requirement {
    Green(String),
    Yellow(String),
    Black(String),
}

fn word_matches_requirements<N: AsRef<str>>(word: N, colors: &[Requirement; 5]) -> bool {
    let word_ref = word.as_ref();
    let green_chars = colors
        .into_iter()
        .enumerate()
        .filter(|(_, c)| match c {
            Requirement::Green(_) => true,
            _ => false,
        })
        .collect();
    for (i, color) in colors.iter().enumerate() {
        if !word_matches_requirement(word_ref, i, color, &green_chars) {
            return false;
        }
    }
    return true;
}

fn word_matches_requirement<N: AsRef<str>>(
    word: N,
    index: usize,
    req: &Requirement,
    greens: &Vec<(usize, &Requirement)>,
) -> bool {
    let word = word.as_ref();
    match req {
        Requirement::Green(char) => {
            let char_at_index = &word[index..index + 1];
            return char_at_index.eq(char);
        }
        Requirement::Yellow(char) => {
            if !word.contains(char) {
                return false;
            }
            let char_at_index = &word[index..index + 1];
            return !char_at_index.eq(char);
        }
        Requirement::Black(char) => {
            let mut found_green_match = false;
            for (green_index, req) in greens {
                match req {
                    Requirement::Green(green_char) => {
                        found_green_match = *green_index != index && green_char == char;
                    }
                    _ => break,
                }
                if found_green_match {
                    break;
                }
            }
            if !found_green_match {
                return !word.contains(char);
            } else {
                return true;
            }
        }
    }
}

fn create_word_list() -> Vec<String> {
    let words_string = include_str!("../sortedwords.txt");
    let lines = words_string.split("\n");
    lines.into_iter().map(|s| s.to_string()).collect()
}

fn calculate_score<N: AsRef<str>, M: AsRef<str>>(word: N, target: M) -> u32 {
    let word = word.as_ref();
    let target = target.as_ref();
    let has_doubles = has_doubles(word);
    let mut sum = 0;
    for (index, char) in word.chars().into_iter().enumerate() {
        let req = char_requirement(index, char.to_string(), target);
        sum += match req {
            Requirement::Green(_) => 3,
            Requirement::Yellow(_) => {
                if has_doubles {
                    0
                } else {
                    2
                }
            }
            Requirement::Black(_) => 0,
        }
    }
    return sum;
}

fn char_requirement<N: AsRef<str>, M: AsRef<str>>(index: usize, char: N, target: M) -> Requirement {
    let target = target.as_ref();
    let char = char.as_ref();
    if !target.contains(char) {
        return Requirement::Black(char.to_string());
    } else {
        if target[index..index + 1].eq(char) {
            return Requirement::Green(char.to_string());
        } else {
            return Requirement::Yellow(char.to_string());
        }
    }
}

fn has_doubles<N: AsRef<str>>(word: N) -> bool {
    let mut set = HashSet::new();
    let word = word.as_ref();
    for char in word.chars() {
        if set.contains(&char) {
            return true;
        } else {
            set.insert(char);
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_score() {
        let score = calculate_score("cares", "cotte");
        assert_eq!(4, score);

        let score = calculate_score("pluto", "cares");
        assert_eq!(0, score);
    }

    #[test]
    fn has_no_blacks() {
        let requirement = [
            Requirement::Black("c".to_string()),
            Requirement::Black("d".to_string()),
            Requirement::Black("e".to_string()),
            Requirement::Black("f".to_string()),
            Requirement::Black("g".to_string()),
        ];
        assert_eq!(true, word_matches_requirements("abbas", &requirement));
        assert_eq!(false, word_matches_requirements("caby", &requirement));
        assert_eq!(false, word_matches_requirements("decks", &requirement));
    }

    #[test]
    fn yellow_is_in_word_but_not_on_location() {
        assert_eq!(
            true,
            word_matches_requirement("dade", 0, &Requirement::Yellow("a".to_string()))
        );
    }

    #[test]
    fn yellow_is_on_location() {
        assert_eq!(
            false,
            word_matches_requirement("dade", 1, &Requirement::Yellow("a".to_string()))
        );
    }

    #[test]
    fn yellow_is_not_in_word() {
        assert_eq!(
            false,
            word_matches_requirement("dade", 3, &Requirement::Yellow("g".to_string()))
        );
    }

    #[test]
    fn green_matches_location() {
        assert_eq!(
            true,
            word_matches_requirement("dade", 0, &Requirement::Green("d".to_string()))
        );

        assert_eq!(
            false,
            word_matches_requirement("dade", 1, &Requirement::Green("d".to_string()))
        );
    }

    #[test]

    fn handles_multiple() {
        let requirement = [
            Requirement::Green("p".to_string()),
            Requirement::Green("l".to_string()),
            Requirement::Yellow("o".to_string()),
            Requirement::Green("t".to_string()),
            Requirement::Black("e".to_string()),
        ];
        assert_eq!(true, word_matches_requirements("pluto", &requirement));
        assert_eq!(false, word_matches_requirements("plate", &requirement));
    }
}
