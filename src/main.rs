use std::io;
use std::collections::HashSet;

fn main() {
    let words = get_input_lines(3);
    let hs = vec_string_hashset(words);
    let hc = string_hashset("hoge".to_string());
    println!("{:?} {:?}", hs, hc);
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_lines(line_len:u32) -> Vec<String> {
    let mut vec:Vec<String> = vec![];
    let mut input:Vec<String>;
    for _ in 0..line_len {
        input = get_input();
        vec.append(&mut input);
    }
    vec
}

fn vec_string_hashset(words:Vec<String>) -> HashSet<String> {
    words.into_iter().collect()
}

fn string_hashset(s:String) -> HashSet<char> {
    let vec:Vec<char> = s.chars().collect();
    vec.into_iter().collect()
}
