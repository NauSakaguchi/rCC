use crate::node::NodeKind::ND_NUM;

pub enum NodeKind {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_NUM,
}

pub struct Node<'a> {
    kind: NodeKind,
    lhs: Option<Box<&'a Node<'a>>>,
    rhs: Option<Box<&'a Node<'a>>>,
    val: Option<isize>,
}

impl <'a> Node <'a> {
    pub fn new(kind: NodeKind, lhs: &'a Node, rhs: &'a Node) -> Self {
        Self {
            kind,
            lhs: Option::from(Box::from(lhs)),
            rhs: Option::from(Box::from(rhs)),
            val: None,
        }
    }

    pub fn new_num(val: isize) -> Self {
        Self {
            kind: ND_NUM,
            lhs: None,
            rhs: None,
            val: Option::from(val),
        }
    }

    fn has_lhs(&self) -> bool {
        match self.lhs {
            Some(_) => true,
            None => false,
        }
    }

    fn has_rhs(&self) -> bool {
        match self.rhs {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_rhs(&self) -> &Node {
        match self.rhs.as_ref() {
            Some(x) => **x,
            None => panic!(),
        }
    }

    pub fn get_lhs(&self) -> &Node {
        match self.lhs.as_ref() {
            Some(x) => **x,
            None => panic!(),
        }
    }

    pub fn get_num(&self) -> isize {
        match self.val {
            Some(x) => x,
            None => panic!(),
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
        let num_lhs = Node::new_num(6);
        let num_rhs = Node::new_num(26);

        let node = Node::new(NodeKind::ND_MUL, &num_lhs, &num_rhs);
        let node2 = Node::new_num(36);
        let node3 = Node::new(NodeKind::ND_ADD, &node, &node2);

        assert_eq!(6, node.get_lhs().get_num());
        assert_eq!(num_lhs.get_num(), node.get_lhs().get_num());

        assert_eq!(26, node.get_rhs().get_num());
        assert_eq!(num_rhs.get_num(), node.get_rhs().get_num());

        assert_eq!(num_lhs.get_num(), node3.get_lhs().get_lhs().get_num());
        assert_eq!(num_rhs.get_num(), node3.get_lhs().get_rhs().get_num());
    }
}