#![allow(non_snake_case)]

use std::env;
use rCC::token::tokenize;
use rCC::{parser, generator};
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect(); // get input as command arguments
    let mut unique_number:usize = 0;

    let program = if args.len() == 1 { //if no argument, then compile test.c file
        let filename = "test.c".to_string();
        let mut file = File::open(filename)
            .expect("file not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        contents
    } else { // else compile the code in the arguments
        args[1..].join(" ")
    };


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

    generator::generator(&nodes, &mut unique_number);
    println!("\tpop rax");


    println!("\tmov rsp, rbp");
    println!("\tpop rbp");
    println!("\tret"); //return RAX value
}
