use scanner::TokenType as Tokens;
use scanner::Token as Token;
use tree::Node as Node;
use tree::ASTNodeKind as ASTType;
use constdata::ConstData as ConstData;

pub struct Parser<'a>
{
    tokens: &'a Vec<Token>,
    index: usize,
    const_data: ConstData
}

impl<'a> Parser<'a>
{
    pub fn new(toks: &Vec<Token>) -> Parser {
        Parser { tokens: toks, index: 0, const_data: ConstData::new() }
    }

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

    fn peek(&self) -> Option<Tokens> {

        if (self.index) >= self.tokens.len() {
            return None
        } else {
            return Some(self.tokens[self.index].get_type())
        }
    }

    fn peek_ahead(&self) -> Option<Tokens> {

        if (self.index + 1) >= self.tokens.len() {
            return None
        } else {
            return Some(self.tokens[self.index + 1].get_type())
        }
    }

    pub fn start(&mut self) -> Vec<Node> {
        let mut ast = vec!();

        let mut last_index = self.index;

        loop {
            last_index = self.index;
            let ass = self.assignment();
            if ass.is_some() && self.terminator() {
                ast.push(ass.unwrap());
            }


            if let Some(current_token) = self.peek() {
                match current_token {
                    Tokens::FunctionCall => {
                        let fun = self.funcall();
                        if fun.is_some() && self.terminator() {
                            ast.push(fun.unwrap());
                        }
                    },
                    _ => {}
                }
            }

            // If we haven't moved, we've either finished iterating or something went wrong.
            if self.index == last_index {
                break;
            }
        }

        return ast;
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
            self.const_data.insert(&t.get_val());
            return Some(self.make_node(ASTType::Integer, Some(t.get_val())));
        } else {
            return None;
        }
    }

    fn assignment(&mut self) -> Option<Node> {

        let v = self.variable();
        if !v.is_some() {
            return None
        }

        let mut x = self.make_node(ASTType::Assignment, None);

        x.append_l(ASTType::Variable, v.unwrap());

        if !self.equals() {
            return None;
        }

        let int = self.integer();
        match int {
            Some(r) => x.append_r(ASTType::Integer, r),
            None    => return None
        }

        // Parse OK, return expr tree
        return Some(x);
    }

    fn funcall(&mut self) -> Option<Node> {
        if let Some(t) = self.consume_token(Tokens::FunctionCall) {
            let funcall = self.make_node(ASTType::FunctionCall, Some(t.get_val()));

            if self.consume(Tokens::ParenOpen) {
                if let Some(function_with_params) = self.append_param_list(funcall) {
                    return Some(function_with_params);
                }
            }
        }

        return None;
    }

    // Recurse over param list, appending function params to right subtree
    fn append_param_list(&mut self, mut node_list: Node) -> Option<Node> {
        if self.consume(Tokens::ParenClose) {
            return Some(node_list);

        } else if let Some(param) = self.consume_token(Tokens::Integer) {
            node_list.append_r(ASTType::Integer, self.make_node(ASTType::Integer, Some(param.get_val())));
            self.const_data.insert(&param.get_val());

            if let Some(right_subtree) = node_list.get_right() {
                self.append_param_list(right_subtree);
                return Some(node_list);
            }
        }

        panic!("Something unexpected is in the function param list");
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

    pub fn get_const_data(&self) -> &ConstData {
        &self.const_data
    }
}
