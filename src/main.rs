#![allow(non_snake_case)]

use std::env;
use rCC::token::tokenize;
use rCC::node::{Node, NodeKind};
use rCC::token::TokenKind::TK_EOF;
use rCC::{parser, generator};

fn main() {
    let mut index = 0;

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: no argument!");
        return ()
    }

    let program = args[1..].join(" ");

    let token_list = tokenize(&program);
    let nodes = parser::parse(&token_list);

    //print an assemble code from here
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generator::gen(&nodes);

    // println!("  mov rax, {}", token_list[index].get_val());
    // index += 1;
    //
    // while !token_list[index].is_end() {
    //     if token_list[index].get_reserved() == "+" {
    //         index += 1;
    //         println!("  add rax, {}", token_list[index].get_val());
    //         index+= 1;
    //     } else if token_list[index].get_reserved() == "-" {
    //         index += 1;
    //         println!("  sub rax, {}", token_list[index].get_val());
    //         index+= 1;
    //     } else {
    //         panic!();
    //     }
    // }

    println!("\tpop rax");
    println!("\tret");
}
