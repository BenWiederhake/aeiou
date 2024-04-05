use std::fmt;
use std::fs::read_to_string;
use std::collections::HashSet;

const WORDLIST_FILENAME: &str = "/usr/share/dict/ngerman";
//const WORDLIST_FILENAME: &str = "/usr/share/dict/american-english";

fn read_words() -> HashSet<String> {
    read_to_string(WORDLIST_FILENAME)
        .unwrap()
        .lines()
        .map(|line| {
            let mut line_string = line.to_string();
            line_string.make_ascii_lowercase();
            line_string
        })
        .collect::<HashSet<_>>()
}

struct CommaSeparated(Vec<String>);

impl fmt::Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter().fuse();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }
        for item in iter {
            write!(f, ", {}", item)?;
        }
        Ok(())
    }
}

fn main() {
    let wordset = read_words();
    let mut matches = 0;
    let mut close = 0;
    for word in wordset.iter() {
        for (offset, _) in word.match_indices('a') {
            let all_words = "aeiouy" // Slightly wasteful, but whatever
                .chars()
                .filter_map(|other_vowel| {
                    let other_word = format!("{}{}{}", &word[..offset], other_vowel, &word[offset + 1..]);
                    if wordset.contains(&other_word) {
                        Some(other_word)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            match all_words.len() {
                6 => {
                    println!("!!! {}", CommaSeparated(all_words));
                    matches += 1;
                }
                5 => {
                    println!("- {}", CommaSeparated(all_words));
                    matches += 1;
                }
                4 => {
                    // println!("# {}", CommaSeparated(all_words));
                    close += 1;
                }
                _ => {}
            }
        }
    }
    println!("Found {matches} matches, {close} close matches.");
}
