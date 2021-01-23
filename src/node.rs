use crate::node::NodeKind::{ND_NUM, ND_ADD, ND_SUB, ND_MUL, ND_DIV, ND_EQ, ND_NEQ, ND_GT, ND_GTE, ND_LT, ND_LTE, ND_ASSIGN, ND_LVAR};

#[allow(non_camel_case_types)]
pub enum NodeKind {
    ND_ADD, // +
    ND_SUB, // -
    ND_MUL, // *
    ND_DIV, // /
    ND_NUM,

    ND_EQ, // ==
    ND_NEQ, // !=
    ND_GT, // >
    ND_GTE, // >=
    ND_LT, // <
    ND_LTE, // <=

    ND_ASSIGN,
    ND_LVAR,
}

pub struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: Option<isize>,
}

impl Node {
    pub fn new(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        let node = Self {
            kind,
            lhs: Option::from(lhs),
            rhs: Option::from(rhs),
            val: None,
        };
        Box::new(node)
    }

    pub fn new_num(val: isize) -> Box<Self> {
        let node = Self {
            kind: ND_NUM,
            lhs: None,
            rhs: None,
            val: Option::from(val),
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