use crate::node::NodeKind::{*};
use crate::node::Node;

pub fn generator(nodes: &Vec<Box<Node>>, unique_number: &mut usize) {
    for node in nodes{
        gen(node, unique_number);
    }
}

pub fn gen(node: &Box<Node>, unique_number: &mut usize) {
    match node.get_kind() {
        ND_BLOCK => {
            generator(node.get_block(), unique_number);
            println!("\tpop rax");
            return;
        }
        ND_RETURN => {
            gen(node.get_lhs(), unique_number);
            println!("\tpop rax");
            println!("\tmov rsp, rbp");
            println!("\tpop rbp");
            println!("\tret");
            return;
        }
        ND_NUM => {
            println!("\tpush {}", node.get_num());
            return;
        }
        ND_CALL => {
            println!("\tcall {}", node.get_name());
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
            gen(node.get_rhs(), unique_number);

            println!("\tpop rdi");
            println!("\tpop rax");
            println!("\tmov [rax], rdi");
            println!("\tpush rdi");
        }
        _ => ()
    }

    match node.get_kind() {
        ND_WHILE => {
            let number = *unique_number;
            *unique_number += 1;
            println!(".Lbegin{}:", number);
            gen(node.get_lhs(), unique_number);
            println!("\tpop rax");
            println!("\tcmp rax, 0");
            println!("\tje .Lend{}", number);
            gen(node.get_rhs(), unique_number);
            println!("\tjmp .Lbegin{}", number);
            println!(".Lend{}:", number);
        }
        ND_FOR => {
            let number = *unique_number;
            *unique_number += 1;
            match node.get_lhs().get_lhs().get_kind() {
                ND_NONE => {}
                _ => gen(node.get_lhs().get_lhs(), unique_number),
                // _ => panic!("Unexpected kind: {}", node.get_lhs().get_lhs().get_kind_as_string()),
            }
            println!(".Lbegin{}:", number);
            match node.get_lhs().get_rhs().get_kind() {
                ND_NONE => {}
                _ => gen(node.get_lhs().get_rhs(), unique_number),
                // _ => panic!("Unexpected kind: {}", node.get_lhs().get_rhs().get_kind_as_string()),
            }
            println!("\tpop rax");
            println!("\tcmp rax, 0");
            println!("\tje .Lend{}", number);
            gen(node.get_rhs().get_rhs(), unique_number);
            match node.get_rhs().get_lhs().get_kind() {
                ND_NONE => {}
                _ => gen(node.get_rhs().get_lhs(), unique_number),
                // _ => panic!("Unexpected kind: {}", node.get_rhs().get_lhs().get_kind_as_string()),
            }
            println!("\tjmp .Lbegin{}", number);
            println!(".Lend{}:", number);
        }
        ND_IF => {
            let number = *unique_number;
            *unique_number += 1;
            gen(node.get_lhs().get_lhs(), unique_number);
            println!("\tpop rax");
            println!("\tcmp rax, 0");
            println!("\tje .Lelse{}", number);
            gen(node.get_lhs().get_rhs(), unique_number);
            println!("\tjmp .Lend{}", number);
            println!(".Lelse{}:", number);
            match node.get_rhs().get_kind() {
                ND_NONE => {}
                _ => gen(node.get_rhs(), unique_number),
                // _ => panic!("Unexpected kind: {}", node.get_rhs().get_lhs().get_kind_as_string()),
            }
            println!(".Lend{}:", number);
        }
        _ => {
            gen(node.get_lhs(), unique_number);
            gen(node.get_rhs(), unique_number);
        }
    }


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
        _ => {},
        // _ => panic!("Unexpected node kind: {}", node.get_kind_as_string())
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