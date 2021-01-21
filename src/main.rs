#![allow(non_snake_case)]


use std::env;
use rCC::token::tokenize;
use rCC::node::{Node, NodeKind};

fn main() {
    let mut index = 0;

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Error: no argument!");
        return ()
    }

    let program = args[1..].join(" ");

    let token_list = tokenize(&program);


    //print an assemble code from here
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  mov rax, {}", token_list[index].get_val());
    index += 1;

    while token_list[index].get_kind() != "TK_EOF" {
        if token_list[index].get_reserved() == "+" {
            index += 1;
            println!("  add rax, {}", token_list[index].get_val());
            index+= 1;
        } else if token_list[index].get_reserved() == "-" {
            index += 1;
            println!("  sub rax, {}", token_list[index].get_val());
            index+= 1;
        } else {
            panic!();
        }
    }

    println!("  ret");
}
