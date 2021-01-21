use crate::node::Node;
use crate::token::Token;
use crate::node::NodeKind::{ND_MUL, ND_DIV, ND_ADD, ND_SUB};

pub fn parse(token_list: &Vec<Token>) -> Box<Node> {
    let mut index: usize = 0;
    expr(token_list, &mut index)
}

fn expr(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    let mut node = mul(token_list, index);

    'expr: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "+" {
                *index += 1; //consume "+"
                node = Node::new(ND_ADD, node, mul(token_list, index));
            } else if token_list[*index].get_reserved() == "-" {
                *index += 1; //consume "-"
                node = Node::new(ND_SUB, node, mul(token_list, index));
            } else {
                break 'expr
            }
        } else {
            break 'expr
        }
    }

    node
}

fn mul(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    let mut node = primary(token_list, index); //consume primary

    'mul: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "*" {
                *index += 1; //consume "*"
                node = Node::new(ND_MUL, node, primary(token_list, index));
            } else if token_list[*index].get_reserved() == "/" {
                *index += 1; //consume "/"
                node = Node::new(ND_DIV, node, primary(token_list, index));
            } else {
                break 'mul
            }
        } else {
            break 'mul
        }
    }

    node
}

fn primary(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {

    if token_list[*index].is_reserved() {
        if token_list[*index].get_reserved() == "(" {
            *index += 1; //consume "("
            let node = expr(token_list, index);
            if token_list[*index].get_reserved() == ")" {
                *index += 1; //consume ")"
                return node;
            } else {
                panic!("Expected \")\", but got {}", token_list[*index].get_reserved());
            }
        } else {
            panic!("Expected \"(\", but got {}", token_list[*index].get_reserved());
        }
    }

    if token_list[*index].is_num() {
        let node = Node::new_num(token_list[*index].get_val());
        *index += 1; //consume num
        node
    } else {
        panic!();
    }
}


#[cfg(test)]
mod tests {
    // use crate::node::NodeKind::ND_ADD;
    // use crate::node::{Node, NodeKind};

    use crate::token::tokenize;
    use crate::parser;
    use crate::node::NodeKind::ND_ADD;

    #[test]
    fn test_for_parser() {
        let mut index = 0;
        let args: Vec<String> = vec!["hoge".to_string(), "1 *2 + 3/ 4".to_string(), "+1".to_string()];

        if args.len() == 1 {
            panic!("Error: no argument!");
        }
        let program = args[1..].join(" ");

        let token_list = tokenize(&program);

        let nodes = parser::parse(&token_list);

        assert_eq!(true, nodes.get_rhs().is_num());


    }
}

