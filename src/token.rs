#![allow(non_snake_case)]

use crate::token::TokenKind::{TK_RESERVED, TK_NUM, TK_EOF};

pub enum TokenKind {
    TK_RESERVED,
    TK_NUM,
    TK_EOF,
}

pub struct Token {
    kind: TokenKind,
    val: Option<isize>,
    reserved: Option<String>,
}

impl Token {
    pub fn new(token_kind: TokenKind, val: Option<isize>, res: Option<String>) -> Self {
        Self {
            kind: token_kind,
            val,
            reserved: res,
        }
    }

    pub fn get_val(&self) -> isize {
        match self.val {
            Some(x) => x,
            None => panic!("Expected number, but no value to return")
        }
    }

    pub fn get_reserved(&self) -> &String {
        match self.reserved.as_ref() {
            Some(x) => x,
            None => panic!("\n\nnote: Expected a reserved word, but no word to return.\nToken Kind: {}\n\n", self.get_kind()),
        }
    }

    pub fn get_kind(&self) -> String {
        match self.kind {
            TK_EOF => "TK_EOF".to_string(),
            TK_NUM => "TK_NUM".to_string(),
            TK_RESERVED => "TK_RESERVED".to_string(),
        }
    }

}

pub fn tokenize(program: &String) -> Vec<Token> {
    let mut index: usize = 0;
    let mut chars:Vec<char> = program.chars().collect();
    let mut token_list = Vec::with_capacity(program.len() / 4);
    let mut val = String::with_capacity(10);

    'tokenize: for (index, character) in chars.iter().enumerate() {
        if *character == ' ' {
            continue 'tokenize
        } else if character.is_numeric() {
            val.push(*character);
            if index + 1 < chars.len() { //if (index + 1) is not at end,
                if !chars[index + 1].is_numeric() { // check if next char is numeric
                    let token = Token::new(TK_NUM, Some(val.parse::<isize>().unwrap()), None);
                    token_list.push(token);
                    val.clear();
                }
            } else {
                let token = Token::new(TK_NUM, Some(val.parse::<isize>().unwrap()), None);
                token_list.push(token);
            }
        } else {
            val.push(*character);
            let token = Token::new(TK_RESERVED, None, Some(val.clone()));
            token_list.push(token);
            val.clear();
        }
    }

    token_list.push(Token::new(TK_EOF, None, None));

    token_list
}