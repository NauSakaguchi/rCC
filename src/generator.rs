use crate::node::NodeKind::{*};
use crate::node::Node;

pub fn generator(nodes: &Vec<Box<Node>>) {
    for node in nodes{
        gen(node);
    }
}

pub fn gen(node: &Box<Node>) {
    match node.get_kind() {
        ND_NUM => {
            println!("\tpush {}", node.get_num());
            return;
        }
        ND_LVAR => {
            gen_lval(node);
            println!("\tpop rax");
            println!("\tmov rax, [rax]");
            println!("\tpush rax");
            return;
        }
        ND_ASSIGN => {
            gen_lval(node.get_lhs());
            gen(node.get_rhs());

            println!("\tpop rdi");
            println!("\tpop rax");
            println!("\tmov [rax], rdi");
            println!("\tpush rdi");
        }
        _ => ()
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
        ND_ASSIGN | ND_LVAR => (),
        _ => panic!("Unexpected node kind: {}", node.get_kind_as_string())
    }

    println!("\tpush rax");

}

fn gen_lval(node: &Box<Node>) {
    match node.get_kind() {
        ND_LVAR => {
            println!("\tmov rax, rbp");
            println!("\tsub rax, {}", node.get_offset());
            println!("\tpush rax");
        }
        _ => panic!("Expected an variable on the left side of \"=\", but got {} (NodeKind)", node.get_kind_as_string())
    }
}