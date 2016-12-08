///let mut t = Node { val: Some("X".to_string()), left: None, right: None };
///t.insert_l(Some("Y".to_string()));
///t.insert_r(Some("Z".to_string()));


#[derive(Debug, Copy, Clone)]
pub enum ASTNodeType {
    Assignment,
    Variable,
    Constant_Int
}

#[derive(Debug)]
pub struct Node
{
    pub kind:  ASTNodeType,
    pub val:   Option<String>,
    pub left:  Option<Box<Node>>,
    pub right: Option<Box<Node>>
}
#[allow(unused)]
impl Node
{
    fn insert(&mut self, insert_left: bool, kind: ASTNodeType, new_val: Option<String>)
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

    pub fn insert_l(&mut self, kind: ASTNodeType, new_val: Option<String>)
    {
        self.insert(true, kind, new_val);
    }

    pub fn insert_r(&mut self, kind: ASTNodeType, new_val: Option<String>)
    {
        self.insert(false, kind, new_val);
    }

    fn append(&mut self, insert_left: bool, kind: ASTNodeType, new_node: Node)
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

    pub fn append_l(&mut self, kind: ASTNodeType, new_node: Node)
    {
        self.append(true, kind, new_node);
    }

    pub fn append_r(&mut self, kind: ASTNodeType, new_node: Node)
    {
        self.append(false, kind, new_node);
    }

    pub fn get_kind(&self) -> ASTNodeType
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
}