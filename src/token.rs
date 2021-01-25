#![allow(non_snake_case)]

use crate::token::TokenKind::{*};

#[allow(non_camel_case_types)]
pub enum TokenKind {
    TK_RESERVED,
    TK_NUM,
    TK_EOF,
    TK_IDENT,
    TK_RETURN,
    TK_KEYWORD
}

pub struct Token {
    kind: TokenKind,
    val: Option<isize>,
    name: Option<String>,
}

impl Token {
    pub fn new(token_kind: TokenKind, val: Option<isize>, name: Option<String>) -> Self {
        Self {
            kind: token_kind,
            val,
            name,
        }
    }

    pub fn get_val(&self) -> isize {
        match self.val {
            Some(x) => x,
            None => panic!("Expected number, but no value to return")
        }
    }

    pub fn get_reserved(&self) -> &String {
        match self.name.as_ref() {
            Some(x) => x,
            None => panic!("\n\nnote: Expected a reserved word, but no word to return.\nToken Kind: {}\n\n", self.get_kind_as_string()),
        }
    }

    pub fn get_id(&self) -> String {
        match self.name.as_ref() {
            Some(x) => x.clone(),
            None => panic!("\n\nnote: Expected ID, but no ID to return.\nToken Kind: {}\n\n", self.get_kind_as_string()),
        }
    }

    pub fn get_keyword(&self) -> &String {
        match self.name.as_ref() {
            Some(x) => x,
            None => panic!("\n\nnote: Expected KEYWORD, but no KEYWORD to return.\nToken Kind: {}\n\n", self.get_kind_as_string()),
        }
    }

    pub fn get_kind_as_string(&self) -> String {
        match self.kind {
            TK_EOF => "TK_EOF".to_string(),
            TK_NUM => "TK_NUM".to_string(),
            TK_RESERVED => "TK_RESERVED".to_string(),
            TK_IDENT => "TK_IDENT".to_string(),
            TK_RETURN => "TK_RETURN".to_string(),
            TK_KEYWORD => "TK_KEYWORD".to_string(),
        }
    }

    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn is_end(&self) -> bool {
        match &self.kind {
            TK_EOF => true,
            _ => false
        }
    }

    pub fn is_reserved(&self) -> bool {
        match &self.kind {
            TK_RESERVED => true,
            _ => false
        }
    }

    pub fn is_num(&self) -> bool {
        match &self.kind {
            TK_NUM => true,
            _ => false
        }
    }

    pub fn is_id(&self) -> bool {
        match &self.kind {
            TK_IDENT => true,
            _ => false
        }
    }

    pub fn is_return(&self) -> bool {
        match &self.kind {
            TK_RETURN => true,
            _ => false
        }
    }

    pub fn is_keyword(&self) -> bool {
        match &self.kind {
            TK_KEYWORD => true,
            _ => false
        }
    }

}

pub fn tokenize(program: &String) -> Vec<Token> {
    let chars:Vec<char> = program.chars().collect();
    let mut index: usize = 0;
    let mut token_list = Vec::with_capacity(program.len() / 4);

    'tokenize: while index < chars.len() {
        let mut word = String::with_capacity(2);

        if chars[index] == ' ' || chars[index] == '\n'  {
            index += 1;
            continue 'tokenize
        } else if chars[index].is_numeric() {
            index = consume_numeric(&mut word, &chars, index);
            let token = Token::new(TK_NUM, Some(word.parse::<isize>().unwrap()), None);
            token_list.push(token);
        } else if is_reserved(&chars[index]) {
            index = consume_reserved(&mut word, &chars, index);
            let token = Token::new(TK_RESERVED, None, Some(word.clone()));
            token_list.push(token);
        } else {
            index = consume_string(&mut word, &chars, index);
            token_list.push(classify_keyword_or_ident(&word));

        }

        word.clear();
    }

    token_list.push(Token::new(TK_EOF, None, None));

    token_list
}

fn consume_numeric(word: &mut String, chars: &Vec<char>, mut index: usize) -> usize {
    word.push(chars[index]);
    index += 1;
    
    'numeric: loop {
        match chars.get(index) {
            Some(x) => {
                if x.is_numeric() {
                    word.push(chars[index]);
                    index += 1;
                } else {
                    break 'numeric
                }
            },
            None => break 'numeric,
        }
    }
    
    index
}

fn consume_reserved(word: &mut String, chars: &Vec<char>, mut index: usize) -> usize {
    word.push(chars[index]);
    index += 1;

    match chars.get(index) {
        Some(x) => {
            match chars[index - 1] {
                '+' => return index,
                '-' => return index,
                '*' => return index,
                '/' | '(' | ')' => return index,
                '=' | '!' => {
                    match x {
                        '=' => {
                            word.push(*x);
                            index += 1;
                            return index
                        }
                        _ => return index
                    };
                },
                '>' | '<' => {
                    match x {
                        '=' => {
                            word.push(*x);
                            index += 1;
                            return index
                        }
                        _ => return index
                    };
                },
                ';' => return index,
                _ => return index,
            };
        },
        None => return index
    };
}

fn is_reserved(ch: &char) -> bool {
    match ch {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '=' => true,
        '!' => true,
        '>' => true,
        '<' => true,
        '(' => true,
        ')' => true,
        ';' => true,
        _ => false,
    }
}

fn consume_string(word: &mut String, chars: &Vec<char>, mut index: usize) -> usize {
    word.push(chars[index]);
    index += 1;

    'string: loop {
        match chars.get(index) {
            Some(x) => {
                if !is_reserved(x) && *x != ' ' && *x != '\n' {
                    word.push(chars[index]);
                    index += 1;
                } else {
                    break 'string
                }
            },
            None => break 'string,
        }
    }
    index
}

fn classify_keyword_or_ident(word: &String) -> Token {
    match &**word {
        "return" => Token::new(TK_RETURN, None, None),

        "if" => Token::new(TK_KEYWORD, None, Some(word.clone())),
        "else" => Token::new(TK_KEYWORD, None, Some(word.clone())),
        "for" => Token::new(TK_KEYWORD, None, Some(word.clone())),
        "while" => Token::new(TK_KEYWORD, None, Some(word.clone())),
        "continue" => Token::new(TK_KEYWORD, None, Some(word.clone())),
        "break" => Token::new(TK_KEYWORD, None, Some(word.clone())),
        "do" => Token::new(TK_KEYWORD, None, Some(word.clone())),
        "goto" => Token::new(TK_KEYWORD, None, Some(word.clone())),

        _ => Token::new(TK_IDENT, None, Some(word.clone()))
    }
}
