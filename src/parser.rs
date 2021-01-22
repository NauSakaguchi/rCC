use crate::node::Node;
use crate::token::Token;
use crate::node::NodeKind::{ND_MUL, ND_DIV, ND_ADD, ND_SUB, ND_EQ, ND_NEQ, ND_GT, ND_GTE, ND_LT, ND_LTE};

pub fn parse(token_list: &Vec<Token>) -> Box<Node> {
    let mut index: usize = 0;
    equality(token_list, &mut index)
}

fn equality(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    let mut node = relational(token_list, index);

    'equality: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "==" {
                *index += 1; //consume "=="
                node = Node::new(ND_EQ, node, relational(token_list, index));
            } else if token_list[*index].get_reserved() == "!=" {
                *index += 1; //consume "!="
                node = Node::new(ND_NEQ, node, relational(token_list, index));
            } else {
                break 'equality
            }
        } else {
            break 'equality
        }
    }

    node
}

fn relational(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    let mut node = add(token_list, index);

    'relational: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "<" {
                *index += 1; //consume "<"
                node = Node::new(ND_GT, node, add(token_list, index));
            } else if token_list[*index].get_reserved() == "<=" {
                *index += 1; //consume "<="
                node = Node::new(ND_GTE, node, add(token_list, index));
            } else if token_list[*index].get_reserved() == ">" {
                *index += 1; //consume ">"
                node = Node::new(ND_LT, add(token_list, index), node);
            } else if token_list[*index].get_reserved() == ">=" {
                *index += 1; //consume ">="
                node = Node::new(ND_LTE, add(token_list, index), node);
            } else {
                break 'relational
            }
        } else {
            break 'relational
        }
    }

    node
}

fn add(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    let mut node = mul(token_list, index);

    'add: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "+" {
                *index += 1; //consume "+"
                node = Node::new(ND_ADD, node, mul(token_list, index));
            } else if token_list[*index].get_reserved() == "-" {
                *index += 1; //consume "-"
                node = Node::new(ND_SUB, node, mul(token_list, index));
            } else {
                break 'add
            }
        } else {
            break 'add
        }
    }

    node
}

fn mul(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    let mut node = unary(token_list, index);

    'mul: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "*" {
                *index += 1; //consume "*"
                node = Node::new(ND_MUL, node, unary(token_list, index));
            } else if token_list[*index].get_reserved() == "/" {
                *index += 1; //consume "/"
                node = Node::new(ND_DIV, node, unary(token_list, index));
            } else {
                break 'mul
            }
        } else {
            break 'mul
        }
    }

    node
}

fn unary(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    if token_list[*index].is_reserved() {
        if token_list[*index].get_reserved() == "+" {
            *index += 1; //consume "+"
            return primary(token_list, index)
        } else if token_list[*index].get_reserved() == "-" {
            *index += 1; //consume "-"
            return Node::new(ND_SUB, Node::new_num(0), primary(token_list, index));
        }
    }

    primary(token_list, index)
}

fn primary(token_list: &Vec<Token>, index: &mut usize) -> Box<Node> {
    if token_list[*index].is_reserved() {
        if token_list[*index].get_reserved() == "(" {
            *index += 1; //consume "("
            let node = add(token_list, index);
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
        panic!("This token is not ND_NUM nor ND_RESERVED.\nThis token.kind is: {}",
               token_list[*index].get_kind_as_string());
    }
}


#[cfg(test)]
mod tests {
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

