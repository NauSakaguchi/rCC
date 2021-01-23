use crate::node::NodeKind::{ND_NUM, ND_ADD, ND_SUB, ND_MUL, ND_DIV, ND_EQ, ND_NEQ, ND_GT, ND_GTE, ND_LT, ND_LTE, ND_ASSIGN, ND_LVAR};
use crate::node::Node;

pub fn generator(nodes: &Vec<Box<Node>>) {
    for node in nodes{
        gen(node);
    }
}

pub fn gen(node: &Box<Node>) {
    if node.is_num() {
        println!("\tpush {}", node.get_num());
        return;
    }

    gen(node.get_lhs());
    gen(node.get_rhs());

    println!("\tpop rdi");
    println!("\tpop rax");

    match node.get_kind() {
        ND_ADD => println!("\tadd rax, rdi"),
        ND_SUB => println!("\tsub rax, rdi"),
        ND_MUL => println!("\timul rax, rdi"),
        ND_DIV => {
            println!("\tcqo");
            println!("\tidiv rdi");
        },
        ND_EQ => {
            println!("\tcmp rax, rdi");
            println!("\tsete al");
            println!("\tmovzb rax, al");
        },
        ND_NEQ => {
            println!("\tcmp rax, rdi");
            println!("\tsetne al");
            println!("\tmovzb rax, al");
        },
        ND_GT => {
            println!("\tcmp rax, rdi");
            println!("\tsetl al");
            println!("\tmovzb rax, al");
        },
        ND_GTE => {
            println!("\tcmp rax, rdi");
            println!("\tsetle al");
            println!("\tmovzb rax, al");
        },
        ND_LT => {
            println!("\tcmp rax, rdi");
            println!("\tsetl al");
            println!("\tmovzb rax, al");
        },
        ND_LTE => {
            println!("\tcmp rax, rdi");
            println!("\tsetle al");
            println!("\tmovzb rax, al");
        },

        _ => panic!("Unexpected node kind")
    }

    println!("\tpush rax");

}