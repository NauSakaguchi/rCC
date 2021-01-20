#![allow(non_snake_case)]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("no argument!");
        return ()
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", args[1]);
    println!("  ret");
}
