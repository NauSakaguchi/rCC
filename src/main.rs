#![allow(non_snake_case)]


use std::env;
use rCC::token;
use rCC::token::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: no argument!");
        return ()
    }

    let program = args[1..].join(" ");

    let token_list = tokenize(&program);

    // let token1 = token::Token::new(TOKEN_NUM, Some(10), None);
    // println!("{}", program);

    // for arg in args.iter() {
    //     program.push(arg);
    // }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");


    println!("  mov rax, {}", token_list[0].get_val());




    println!("  ret");
}
