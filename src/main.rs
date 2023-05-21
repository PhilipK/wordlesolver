use std::{
    collections::HashMap,
    io::{self, BufRead},
    time::SystemTime, fs::File,
};

use unicode_segmentation::{UnicodeSegmentation, Graphemes};

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
    println!();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if current_word.is_none() {
            current_word = Some(line);
            println!("Write your colors (b=black,y=yellow,g=green) then hit enter");
            continue;
        }
        if current_colors.is_none() {
            current_colors = Some(line);

            let word = current_word.unwrap();
            let input = current_colors.unwrap();

            let requirements = string_to_requirements(input, &word).unwrap();
            words.retain(|word| word_matches_requirements(word, &requirements));

            sort_list_by_score(&mut words);
            current_colors = None;
            current_word = None;
            println!("{} possible words remaining", words.len());
            println!("Try one of theese");
            for word in words.iter().take(5) {
                println!("{}", word);
            }
            println!();

            println!("Write your guess (5 letter limit, then hit enter)");
        }
    }
}

fn sort_list_by_score(words: &mut [String]) {
    let now = SystemTime::now();
    let score_map = calc_score_map(words);
    println!("calced scores {:?}", now.elapsed());
    words.sort_by_cached_key(|k| score_map.get(k));
    words.reverse();
    println!("{:?}", now.elapsed())
}

fn calc_score_map(words: &[String]) -> HashMap<String, u32> {
    let mut score_map = HashMap::new();
    for current_word in words.iter() {
        let mut word_score = 0;
        let current_word_graph= current_word.graphemes(true);
        for target in words.iter() {
            word_score += calculate_score_graphemes(current_word_graph.clone(), target.graphemes(true));
        }
        score_map.insert(current_word.to_owned(), word_score);
    }
    score_map
}

fn string_to_requirements<N: AsRef<str>>(input: N, word: &str) -> Result<[Requirement; 5], String> {
    let input = input.as_ref();

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

fn to_requirement<'a>(word: &'a str, index: usize, char: &'_ str) -> Requirement<'a> {
    let s = &word[index..index + 1];
    match char {
        "g" => Requirement::Green(s),
        "y" => Requirement::Yellow(s),
        "b" => Requirement::Black(s),
        _ => panic!("unknown color"),
    }
}

enum Requirement<'a> {
    Green(&'a str),
    Yellow(&'a str),
    Black(&'a str),
}

fn word_matches_requirements<N: AsRef<str>>(word: N, colors: &[Requirement; 5]) -> bool {
    let word_ref = word.as_ref();
    let green_chars = colors
        .iter()
        .enumerate()
        .filter(|(_, c)| matches!(c, Requirement::Green(_)))
        .collect();
    for (i, color) in colors.iter().enumerate() {
        if !word_matches_requirement(word_ref, i, color, &green_chars) {
            return false;
        }
    }
    true
}

fn word_matches_requirement<N: AsRef<str>>(
    word: N,
    index: usize,
    req: &Requirement,
    greens: &Vec<(usize, &Requirement)>,
) -> bool {
    let word = word.as_ref();
    let word = word.graphemes(true).collect::<Vec<&str>>();
    match req {
        Requirement::Green(char) => {
            let char_at_index = &word[index..index + 1];
            char_at_index.join("").eq(char)
        }
        Requirement::Yellow(char) => {
            if !word.contains(char) {
                return false;
            }
            let char_at_index = &word[index..index + 1];
            !char_at_index.join("").eq(char)
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
                !word.contains(char)
            } else {
                true
            }
        }
    }
}

fn create_word_list() -> Vec<String> {
    let words_string = include_str!("../sorteredeord.txt");
    let lines = words_string.split('\n');
    let mut words: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
    //sort_list_by_score(&mut words);
    //for word in words.iter(){
        //println!("{}",word);
    //}

    //println!("");
    words
}

fn calculate_score_graphemes(word: Graphemes, target: Graphemes) -> u32 {
    let has_doubles = has_doubles_graphemes(word.clone());
    let mut sum = 0;
    for (index, char) in word.enumerate() {
        let req = char_requirement_graphemes(index, char, target.clone());
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
    sum
}


fn calculate_score(word: &str, target: &str) -> u32 {
    let has_doubles = has_doubles(word);
    let mut sum = 0;
    let graphemes = target.graphemes(true);
    for (index, char) in word.graphemes(true).enumerate() {
        let req = char_requirement_graphemes(index, char, graphemes.clone());
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
    sum
}

fn char_requirement_graphemes<'a>(index: usize, char: &'a str, target: Graphemes) -> Requirement<'a> {
    for (i, cur) in target.enumerate() {
        if cur.eq(char) {
            if index == i {
                return Requirement::Green(char);
            } else {
                return Requirement::Yellow(char);
            }
        }
    }
    Requirement::Black(char)
}



fn has_doubles_graphemes(chars: Graphemes) -> bool {
    let chars: Vec<&str> = chars.collect();
    for i in 0..5 {
        for j in i + 1..5 {
            if chars[i] == chars[j] {
                return true;
            }
        }
    }
    false
}


fn has_doubles<N: AsRef<str>>(word: N) -> bool {
    let word = word.as_ref();

    let chars: Vec<&str> = word.graphemes(true).collect();

    for i in 0..5 {
        for j in i + 1..5 {
            if chars[i] == chars[j] {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_score() {
        let score = calculate_score("cares", "cotte");
        assert_eq!(5, score);

        let score = calculate_score("pluto", "cares");
        assert_eq!(0, score);


        let score = calculate_score("æbler", "cares");
        assert_eq!(5, score);


        let score = calculate_score("æbler", "æbler");
        assert_eq!(15, score);
    }

    #[test]
    fn has_no_blacks() {
        let requirement = [
            Requirement::Black("c"),
            Requirement::Black("d"),
            Requirement::Black("e"),
            Requirement::Black("f"),
            Requirement::Black("g"),
        ];
        assert!(word_matches_requirements("abbas", &requirement));
        assert!(!word_matches_requirements("caby", &requirement));
        assert!(!word_matches_requirements("decks", &requirement));
    }

    #[test]
    fn yellow_is_in_word_but_not_on_location() {
        assert!(word_matches_requirement(
            "dade",
            0,
            &Requirement::Yellow("a"),
            &vec![]
        ));
    }

    #[test]
    fn yellow_is_on_location() {
        assert!(!word_matches_requirement(
            "dade",
            1,
            &Requirement::Yellow("a"),
            &vec![]
        ));
    }

    #[test]
    fn yellow_is_not_in_word() {
        assert!(!word_matches_requirement(
            "dade",
            3,
            &Requirement::Yellow("g"),
            &vec![]
        ));
    }

    #[test]
    fn green_matches_location() {
        assert!(word_matches_requirement(
            "dade",
            0,
            &Requirement::Green("d"),
            &vec![]
        ));

        assert!(!word_matches_requirement(
            "dade",
            1,
            &Requirement::Green("d"),
            &vec![]
        ));
    }

    #[test]

    fn handles_multiple() {
        let requirement = [
            Requirement::Green("p"),
            Requirement::Green("l"),
            Requirement::Yellow("o"),
            Requirement::Green("t"),
            Requirement::Black("e"),
        ];
        assert!(word_matches_requirements("pluto", &requirement));
        assert!(!word_matches_requirements("plate", &requirement));
    }
}
