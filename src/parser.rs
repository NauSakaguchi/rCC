use crate::node::{Node, NodeKind};
use crate::token::Token;
use crate::lvar::LVar;
use crate::node::NodeKind::{*};

pub fn parse(token_list: &Vec<Token>) -> Vec<Box<Node>> {
    let mut nodes: Vec<Box<Node>> = Vec::with_capacity(100);
    let mut index: usize = 0;
    let mut l_vars: Vec<Box<LVar>> = vec![Box::from(LVar::new(" ".to_string(), 0))];

    while !token_list[index].is_end() {
        nodes.push(stmt(token_list, &mut index, &mut l_vars));
    }

    nodes
}

fn stmt(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    let mut node = expr(token_list, index, l_vars);

    if token_list[*index].is_end() {
        //nothing to do
    }else if token_list[*index].get_reserved() == ";" {
            *index += 1; //consume ";"
    } else {
            panic!("Expected \";\" or \"eof\", but got {} (TokenKind)", token_list[*index].get_kind_as_string());
    }

    node
}

fn expr(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    let mut node = assign(token_list, index, l_vars);
    node
}

fn assign(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    let mut node = equality(token_list, index, l_vars);

    if token_list[*index].is_reserved() {
        if token_list[*index].get_reserved() == "=" {
            *index += 1; //consume "="
            node = Node::new(ND_ASSIGN, node, assign(token_list, index, l_vars))
        }
    }

    node
}

fn equality(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    let mut node = relational(token_list, index, l_vars);

    'equality: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "==" {
                *index += 1; //consume "=="
                node = Node::new(ND_EQ, node, relational(token_list, index, l_vars));
            } else if token_list[*index].get_reserved() == "!=" {
                *index += 1; //consume "!="
                node = Node::new(ND_NEQ, node, relational(token_list, index, l_vars));
            } else {
                break 'equality
            }
        } else {
            break 'equality
        }
    }

    node
}

fn relational(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    let mut node = add(token_list, index, l_vars);

    'relational: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "<" {
                *index += 1; //consume "<"
                node = Node::new(ND_GT, node, add(token_list, index, l_vars));
            } else if token_list[*index].get_reserved() == "<=" {
                *index += 1; //consume "<="
                node = Node::new(ND_GTE, node, add(token_list, index, l_vars));
            } else if token_list[*index].get_reserved() == ">" {
                *index += 1; //consume ">"
                node = Node::new(ND_LT, add(token_list, index, l_vars), node);
            } else if token_list[*index].get_reserved() == ">=" {
                *index += 1; //consume ">="
                node = Node::new(ND_LTE, add(token_list, index, l_vars), node);
            } else {
                break 'relational
            }
        } else {
            break 'relational
        }
    }

    node
}

fn add(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    let mut node = mul(token_list, index, l_vars);

    'add: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "+" {
                *index += 1; //consume "+"
                node = Node::new(ND_ADD, node, mul(token_list, index, l_vars));
            } else if token_list[*index].get_reserved() == "-" {
                *index += 1; //consume "-"
                node = Node::new(ND_SUB, node, mul(token_list, index, l_vars));
            } else {
                break 'add
            }
        } else {
            break 'add
        }
    }

    node
}

fn mul(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    let mut node = unary(token_list, index, l_vars);

    'mul: loop {
        if token_list[*index].is_reserved() {
            if token_list[*index].get_reserved() == "*" {
                *index += 1; //consume "*"
                node = Node::new(ND_MUL, node, unary(token_list, index, l_vars));
            } else if token_list[*index].get_reserved() == "/" {
                *index += 1; //consume "/"
                node = Node::new(ND_DIV, node, unary(token_list, index, l_vars));
            } else {
                break 'mul
            }
        } else {
            break 'mul
        }
    }

    node
}

fn unary(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    if token_list[*index].is_reserved() {
        if token_list[*index].get_reserved() == "+" {
            *index += 1; //consume "+"
            return primary(token_list, index, l_vars)
        } else if token_list[*index].get_reserved() == "-" {
            *index += 1; //consume "-"
            return Node::new(ND_SUB, Node::new_num(0), primary(token_list, index, l_vars));
        }
    }

    primary(token_list, index, l_vars)
}

fn primary(token_list: &Vec<Token>, index: &mut usize, l_vars: &mut Vec<Box<LVar>>) -> Box<Node> {
    if token_list[*index].is_reserved() {
        if token_list[*index].get_reserved() == "(" {
            *index += 1; //consume "("
            let node = add(token_list, index, l_vars);
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

    if token_list[*index].is_id() {
        let l_var_name = token_list[*index].get_id();
        let l_var_index = find_lvar(&l_var_name, l_vars);
        return match l_var_index {
            Some(x) => {
                let node = Node::new_lval(l_vars[x].get_offset());
                *index += 1;
                node
            }
            None => {
                let l_var = LVar::new(l_var_name, l_vars.last().unwrap().get_offset() + 8);
                let node = Node::new_lval(l_var.get_offset());
                *index += 1;
                l_vars.push(Box::from(l_var));
                node
            }
        }
    }

    if token_list[*index].is_num() {
        let node = Node::new_num(token_list[*index].get_val());
        *index += 1; //consume num
        return node;
    } else {
        panic!("This token is not ND_NUM nor ND_RESERVED.\nThis token.kind is: {}",
               token_list[*index].get_kind_as_string());
    }
}

fn find_lvar(l_var_name: &String, l_vars: &Vec<Box<LVar>>) -> Option<usize> {
    for (index, l_var) in l_vars.iter().enumerate() {
       if l_var_name == l_var.get_name() {
           return Some(index)
       }
    }
    None
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

