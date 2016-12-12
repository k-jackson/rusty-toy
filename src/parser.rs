use scanner::TokenType as Tokens;
use scanner::Token as Token;
use tree::Node as Node;
use tree::ASTNodeType as ASTType;

struct Parser<'a>
{
    tokens: &'a Vec<Token>,
    index: usize
}

impl<'a> Parser<'a>
{
    // todo: split up a peek from consume(), for terminals?
    fn consume(&mut self, token: Tokens) -> bool {

        if self.index >= self.tokens.len() {
            return false;

        } else if self.tokens[self.index].is_type(token) {
            self.index += 1;

            return true;

        } else {
            return false;
        }
    }

    fn consume_token(&mut self, token: Tokens) -> Option<Token> {

        if self.index >= self.tokens.len() {
            return None;

        } else if self.tokens[self.index].is_type(token) {
            self.index += 1;

            return Some(self.tokens[self.index - 1].clone());

        } else {
            return None;
        }
    }

    fn start(&mut self) -> Option<Node> {
        let ass = self.assignment();
        if ass.is_some() && self.terminator() {
            return ass;
        }

        let fun = self.funcall();
        if fun.is_some() && self.terminator() {
            return fun;
        }

        return None;
    }

    fn error(&self) {
        panic!("died");
    }

    fn variable(&mut self) -> Option<Node> {

        if let Some(t) = self.consume_token(Tokens::Variable) {
            return Some(self.make_node(ASTType::Variable, Some(t.get_val())));
        } else {
            return None;
        }
    }

    fn equals(&mut self) -> bool {
        if self.consume(Tokens::EqualSign) {
            return true;
        } else {
            return false;
        }
    }

    fn integer(&mut self) -> Option<Node> {
        if let Some(t) = self.consume_token(Tokens::Integer) {
            return Some(self.make_node(ASTType::ConstantInt, Some(t.get_val())));
        } else {
            return None;
        }
    }

    fn assignment(&mut self) -> Option<Node> {

        let mut x = self.make_node(ASTType::Assignment, None);

        let v = self.variable();
        match v {
            Some(l) => x.append_l(ASTType::Variable, l),
            None    => return None
        }

        if !self.equals() {
            return None;
        }

        let int = self.integer();
        match int {
            Some(r) => x.append_r(ASTType::ConstantInt, r),
            None    => return None
        }

        // Parse OK, return expr tree
        return Some(x);
    }

    fn funcall(&mut self) -> Option<Node> {
        if let Some(t) = self.consume_token(Tokens::FunctionCall) {
            let mut funcall = self.make_node(ASTType::FunctionCall, Some(t.get_val()));

            if self.consume(Tokens::ParenOpen) {
                if let Some(function_with_params) = self.append_param_list(funcall) {
                    return Some(function_with_params);
                }
            }
        }

        return None;
    }

    // Recurse over param list, appending function params to left subtree (for now)
    fn append_param_list(&mut self, mut node_list: Node) -> Option<Node> {
        if self.consume(Tokens::ParenClose) {
            return Some(node_list);

        } else if let Some(param) = self.consume_token(Tokens::Integer) {
            node_list.append_l(ASTType::ConstantInt, self.make_node(ASTType::ConstantInt, Some(param.get_val())));

            if let Some(left_subtree) = node_list.get_left() {
                self.append_param_list(left_subtree);
                return Some(node_list);
            }
        }

        panic!("Something unexpected is in the function param list");
        return None;
    }

    fn is_builtin(&mut self) {
        unimplemented!();
    }

    fn terminator(&mut self) -> bool {
        if self.consume(Tokens::Terminator) {
            return true;
        } else {
            return false;
        }
    }

    fn make_node<'b>(&'b self, ast_type: ASTType, node_val: Option<String>) -> Node {
        return Node {kind: ast_type, val: node_val, left: None, right: None};
    }
}

pub fn parse(tokens: &Vec<Token>) -> Option<Node>
{
    let mut p = Parser { tokens: tokens, index: 0 };
    let parse_result = p.start();

    println!("{:#?}",parse_result);
    if parse_result.is_some() {
        return parse_result;
    } else {
        println!("Parsing failed!");
        return None;
    }
}


