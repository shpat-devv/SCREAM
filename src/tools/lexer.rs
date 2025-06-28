/*
TODO:
----------------------

read file
    --> Read through each line word by word
    --> Check if word is syntax 
    --> Generate token 


iterate through file
check for syntax inside of file
convert syntax of file to tokens
*/

use std::env;
use std::fs;

struct Token {
    keyword: String,
    value: String,
}

pub fn read_program(filepath: &str){
    let content = fs::read_to_string(filepath)
        .expect("File does not exist or is corrupt");

    tokenize(content);
}

fn tokenize(file: String) {
    for line in file.lines() {
        if let Some(idx) = line.find("//") {
            let comment_value = line[idx + 2..].trim().to_string();
            let token = Token {
                keyword: "comment".to_string(),
                value: comment_value,
            };
            add_token_struct(token);
        }
    }
}

fn add_token_struct(token: Token) {
    let mut tokens = Vec::new();
    tokens.push(token);
}

