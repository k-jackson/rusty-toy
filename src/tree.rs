///let mut t = Node { val: Some("X".to_string()), left: None, right: None };
///t.insert_l(Some("Y".to_string()));
///t.insert_r(Some("Z".to_string()));


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ASTNodeKind {
    Assignment,
    Variable,
    Integer,
    FunctionCall
}

#[derive(Debug, Clone)]
pub struct Node
{
    pub kind: ASTNodeKind,
    pub val:   Option<String>,
    pub left:  Option<Box<Node>>,
    pub right: Option<Box<Node>>
}
#[allow(unused)]
impl Node
{
    fn insert(&mut self, insert_left: bool, kind: ASTNodeKind, new_val: Option<String>)
    {
        let target_node = if insert_left { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(insert_left, kind, new_val),
            &mut None => {
                let new_node = Node { kind:kind, val: new_val, left: None, right: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    pub fn insert_l(&mut self, kind: ASTNodeKind, new_val: Option<String>)
    {
        self.insert(true, kind, new_val);
    }

    pub fn insert_r(&mut self, kind: ASTNodeKind, new_val: Option<String>)
    {
        self.insert(false, kind, new_val);
    }

    fn append(&mut self, insert_left: bool, kind: ASTNodeKind, new_node: Node)
    {
        let target_node = if insert_left { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.append(insert_left, kind, new_node),
            &mut None => {
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    pub fn append_l(&mut self, kind: ASTNodeKind, new_node: Node)
    {
        self.append(true, kind, new_node);
    }

    pub fn append_r(&mut self, kind: ASTNodeKind, new_node: Node)
    {
        self.append(false, kind, new_node);
    }

    pub fn get_kind(&self) -> ASTNodeKind
    {
        // Copy-out for now
        return self.kind;
    }

    pub fn has_val(&self) -> bool
    {
        return self.val.is_some();
    }

    pub fn has_left(&self) -> bool
    {
        return self.left.is_some();
    }

    pub fn has_right(&self) -> bool
    {
        return self.right.is_some();
    }

    pub fn get_left(&self) -> Option<Node>
    {
        if let Some(left) = self.left.clone() {
            let unboxed_left: Node = *left;
            Some(unboxed_left)
        } else {
            None
        }
    }

    pub fn get_right(&self) -> Option<Node>
    {
        if let Some(right) = self.right.clone() {
            let unboxed_right: Node = *right;
            Some(unboxed_right)
        } else {
            None
        }
    }

    pub fn traverse_preorder(&self) -> Vec<&Node>
    {
        let mut res: Vec<&Node> = Vec::new();
        let mut stack: Vec<&Node> = Vec::new();

        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node);
            match node.right {
                None => {},
                Some(box ref n) => stack.push(n)
            }
            match node.left {
                None => {},
                Some(box ref n) => stack.push(n)
            }
        }

        res
    }

    pub fn traverse_postorder(&self) -> Vec<&Node>
    {
        let mut res: Vec<&Node> = Vec::new();
        let mut stack: Vec<&Node> = Vec::new();

        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            match node.left {
                None => {},
                Some(box ref n) => stack.push(n)
            }
            match node.right {
                None => {},
                Some(box ref n) => stack.push(n)
            }
            res.push(node);
        }

        let rev_iter = res.iter().rev();
        let mut rev: Vec<&Node> = Vec::new();
        for elem in rev_iter {
            rev.push(elem);
        }

        rev
    }
}