use crate::node::NodeKind::{*};

#[allow(non_camel_case_types)]
pub enum NodeKind {
    ND_ADD, // +
    ND_SUB, // -
    ND_MUL, // *
    ND_DIV, // /
    ND_NUM, // Numeric

    ND_EQ, // ==
    ND_NEQ, // !=
    ND_GT, // >
    ND_GTE, // >=
    ND_LT, // <
    ND_LTE, // <=

    ND_ASSIGN, // =
    ND_LVAR, //local variable

    ND_RETURN, //return

    ND_IF, // if
    ND_WHILE, // while
    ND_FOR, //for

    ND_OTHER,
    ND_NONE,

    ND_BLOCK
}

pub struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: Option<isize>,
    offset: Option<isize>,
    block: Option<Box<Vec<Box<Node>>>>,
}

impl Node {
    pub fn new(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        let node = Self {
            kind,
            lhs: Option::from(lhs),
            rhs: Option::from(rhs),
            val: None,
            offset: None,
            block: None,
        };
        Box::new(node)
    }

    pub fn new_num(val: isize) -> Box<Self> {
        let node = Self {
            kind: ND_NUM,
            lhs: None,
            rhs: None,
            val: Option::from(val),
            offset: None,
            block: None,
        };
        Box::new(node)
    }

    pub fn new_lval(offset: isize) -> Box<Self> {
        let node = Self{
            kind: ND_LVAR,
            lhs: None,
            rhs: None,
            val: None,
            offset: Option::from(offset),
            block: None,
        };
        Box::new(node)
    }

    pub fn new_return(lhs: Box<Node>) -> Box<Self> {
        let node = Self {
            kind: ND_RETURN,
            lhs: Option::from(lhs),
            rhs: None,
            val: None,
            offset: None,
            block: None,
        };
        Box::new(node)
    }

    pub fn new_other(lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Box<Self> {
        let node = Self {
            kind: ND_OTHER,
            lhs,
            rhs,
            val: None,
            offset: None,
            block: None,
        };
        Box::new(node)
    }

    pub fn new_none() -> Box<Self> {
        let node = Self {
            kind: ND_NONE,
            lhs: None,
            rhs: None,
            val: None,
            offset: None,
            block: None,
        };
        Box::new(node)
    }

    pub fn new_block(block: Box<Vec<Box<Node>>>) -> Box<Self> {
        let node = Self {
            kind: ND_BLOCK,
            lhs: None,
            rhs: None,
            val: None,
            offset: None,
            block: Option::from(block),
        };
        Box::new(node)
    }

    pub fn has_lhs(&self) -> bool {
        match self.lhs {
            Some(_) => true,
            None => false,
        }
    }

    pub fn has_rhs(&self) -> bool {
        match self.rhs {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_rhs(&self) -> &Box<Node> {
        match self.rhs.as_ref() {
            Some(x) => x,
            None => panic!(),
        }
    }

    pub fn get_lhs(&self) -> &Box<Node> {
        match self.lhs.as_ref() {
            Some(x) => x,
            None => panic!(),
        }
    }

    pub fn get_block(&self) -> &Vec<Box<Node>> {
        match self.block.as_ref() {
            Some(x) => &**x,
            None => panic!(),
        }
    }

    pub fn get_num(&self) -> isize {
        match self.val {
            Some(x) => x,
            None => panic!(),
        }
    }

    pub fn get_kind(&self) -> NodeKind {
        match self.kind {
            ND_NUM => ND_NUM,
            ND_ADD => ND_ADD,
            ND_SUB => ND_SUB,
            ND_MUL => ND_MUL,
            ND_DIV => ND_DIV,
            ND_EQ => ND_EQ,
            ND_NEQ=> ND_NEQ,
            ND_GT=> ND_GT,
            ND_GTE=> ND_GTE,
            ND_LT => ND_LT,
            ND_LTE=> ND_LTE,
            ND_ASSIGN=> ND_ASSIGN,
            ND_LVAR=> ND_LVAR,
            ND_RETURN=> ND_RETURN,
            ND_IF => ND_IF,
            ND_WHILE => ND_WHILE,
            ND_FOR => ND_FOR,
            ND_OTHER => ND_OTHER,
            ND_NONE => ND_NONE,
            ND_BLOCK => ND_BLOCK,
        }
    }

    pub fn get_kind_as_string(&self) -> String {
        match self.kind {
            ND_NUM => "ND_NUM".to_string(),
            ND_ADD => "ND_ADD".to_string(),
            ND_SUB => "ND_SUB".to_string(),
            ND_MUL => "ND_MUL".to_string(),
            ND_DIV => "ND_DIV".to_string(),
            ND_EQ => "ND_EQ".to_string(),
            ND_NEQ=> "ND_NEQ".to_string(),
            ND_GT=> "ND_GT".to_string(),
            ND_GTE=> "ND_GTE".to_string(),
            ND_LT => "ND_LT".to_string(),
            ND_LTE=> "ND_LTE".to_string(),
            ND_ASSIGN=> "ND_ASSIGN".to_string(),
            ND_LVAR=> "ND_LVAR".to_string(),
            ND_RETURN=> "ND_RETURN".to_string(),
            ND_IF => "ND_IF".to_string(),
            ND_WHILE => "ND_WHILE".to_string(),
            ND_FOR => "ND_FOR".to_string(),
            ND_OTHER => "ND_OTHER".to_string(),
            ND_NONE => "ND_NONE".to_string(),
            ND_BLOCK => "ND_BLOCK".to_string(),
        }
    }

    pub fn get_offset(&self) -> isize {
        match self.offset {
            Some(x) => x,
            None => panic!("This Node does NOT have offset, {} (NodeKind)", self.get_kind_as_string())
        }
    }

    pub fn is_num(&self) -> bool {
        match self.kind {
            ND_NUM => true,
            _ => false,
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::node::NodeKind::ND_ADD;
    use crate::node::{Node, NodeKind};

    #[test]

    fn test_for_Node() {
        //test
        let num_lhs = Node{
            kind: NodeKind::ND_NUM,
            lhs: None,
            rhs: None,
            val: Option::from(6),
        };
        let num_rhs = Node{
            kind: NodeKind::ND_NUM,
            lhs: None,
            rhs: None,
            val: Option::from(26),
        };

        let node = Node::new(NodeKind::ND_MUL, Box::new(num_lhs), Box::new(num_rhs));
        let node2 = Node::new_num(36);
        let node3 = Node::new(NodeKind::ND_ADD, node, node2);

        // assert_eq!(6, node.get_lhs().get_num());
        // assert_eq!(num_lhs.get_num(), node.get_lhs().get_num());
        //
        // assert_eq!(26, node.get_rhs().get_num());
        // assert_eq!(num_rhs.get_num(), node.get_rhs().get_num());

        assert_eq!(6, node3.get_lhs().get_lhs().get_num());
        assert_eq!(26, node3.get_lhs().get_rhs().get_num());
        assert_eq!(36, node3.get_rhs().get_num());
    }
}