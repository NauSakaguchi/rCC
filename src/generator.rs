use crate::node::NodeKind::{ND_NUM, ND_ADD, ND_SUB, ND_MUL, ND_DIV};
use crate::node::Node;

pub fn gen(nodes: &Box<Node>) {
    if nodes.is_num() {
        println!("\tpush {}", nodes.get_num());
        return;
    }

    gen(nodes.get_lhs());
    gen(nodes.get_rhs());

    println!("\tpop rdi");
    println!("\tpop rax");

    match nodes.get_kind() {
        ND_ADD => println!("\tadd rax, rdi"),
        ND_SUB => println!("\tsub rax, rdi"),
        ND_MUL => println!("\timul rax, rdi"),
        ND_DIV => {
            println!("\tcqo");
            println!("\tidiv rdi");
        },
        _ => panic!()
    }

    println!("\tpush rax");

}