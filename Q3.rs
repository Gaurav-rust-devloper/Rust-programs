fn find_shortest_word(s: &str) -> Option<&str> {
    let words = s.split_whitespace();
    let mut shortest_word: Option<&str> = None;

    for word in words {
        if shortest_word.is_none() || word.len() < shortest_word.unwrap().len() {
            shortest_word = Some(word);
        }
    }

    shortest_word
}

fn main() {
    let s = "The quick brown fox jumps over the lazy dog";
    match find_shortest_word(s) {
        Some(word) => println!("The shortest word is: {}", word),
        None => println!("No words found in the input string"),
    }
}
 