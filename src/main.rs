#![allow(non_snake_case)]

use std::env;
use rCC::token::tokenize;
use rCC::{parser, generator};

fn main() {
    let args: Vec<String> = env::args().collect(); // get input as command arguments
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

    //prologue
    //reserve for 26 variables (26 * 8 = 208)
    println!("\tpush rbp");
    println!("\tmov rbp, rsp");
    println!("\tsub rsp, 208");

    generator::generator(&nodes);
    println!("\tpop rax");


    println!("\tmov rsp, rbp");
    println!("\tpop rbp");
    println!("\tret"); //return RAX value
}
