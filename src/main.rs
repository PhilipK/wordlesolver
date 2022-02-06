use std::{
    io::{self, BufRead},
};

fn main() {
    let mut words = create_word_list();
    let stdin = io::stdin();
    let mut current_word: Option<String> = None;
    let mut current_colors: Option<String> = None;

    println!("Write your guess (5 letter limit, then hit enter)");
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

            current_colors = None;
            current_word = None;
            println!("{} possible words remaining", words.len());

            if words.len() < 10 {
                println!("Try one of theese");
                for word in &words {
                    println!("{}", word);
                }
            }
            println!("");

            println!("Write your guess (5 letter limit, then hit enter)");
        }
    }
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
    for (i, color) in colors.iter().enumerate() {
        if !word_matches_requirement(word_ref, i, color) {
            return false;
        }
    }
    return true;
}

fn word_matches_requirement<N: AsRef<str>>(word: N, index: usize, req: &Requirement) -> bool {
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
        Requirement::Black(char) => !word.contains(char),
    }
}

fn create_word_list() -> Vec<String> {
    let words_string = include_str!("../sgb-words.txt");
    let lines = words_string.split("\n");
    lines
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

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
