#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Variable,
    Terminator,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Exponent,
    EqualSign,
    //Equality,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    Integer,
    FunctionCall,
    Unknown
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    t_type: TokenType,
    t_val:  String
}
impl Default for Token {
    fn default() -> Token {
        Token {t_type: TokenType::Unknown, t_val: String::new()}
    }
}
impl Token {
    pub fn is_type(&self, token_type: TokenType) -> bool {
        return self.t_type == token_type;
    }
    pub fn get_val(&self) -> String {
        return self.t_val.clone();
    }

    pub fn get_type(&self) -> TokenType {
        return self.t_type.clone();
    }
}

pub fn scan(program: String) -> Vec<Token> {

    let mut chars = program.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(chr) = chars.next() {
        let tok: Option<Token> = match chr {

            x if x.is_alphabetic() => {

                let mut var_string = String::new();
                let mut sub_match = TokenType::Variable;

                var_string.push(chr);

                loop {
                    match chars.peek() {
                        Some(&next) => {
                            match next {
                                x if x.is_alphabetic() => {
                                    var_string.push(next);
                                    chars.next();
                                },
                                x if x == '(' => {
                                    sub_match = TokenType::FunctionCall;
                                    break
                                }
                                _ => break
                            };
                        },
                        _ => break
                    };
                }

                Some(Token{t_type: sub_match, t_val: var_string})
            },
            x if x.is_numeric() => {
                let mut var_string = String::new();
                var_string.push(chr);

                loop {
                    match chars.peek() {
                        Some(&next) => {
                            match next {
                                x if x.is_numeric() => {
                                    var_string.push(next);
                                    chars.next();
                                },
                                _ => break
                            };
                        },
                        _ => break
                    };
                }

                let num_opt = var_string.parse::<i64>().ok();

                let num_val: i64 = match num_opt {
                    Some(num_opt) => num_opt,
                    None => 0
                };

                Some(Token {t_type: TokenType::Integer, t_val: num_val.to_string()})
            },
            ' ' | '\n'  => None,
            ';' => Some(Token {t_type: TokenType::Terminator, ..Default::default()}),
            '+' => Some(Token {t_type: TokenType::Add, ..Default::default()}),
            '-' => Some(Token {t_type: TokenType::Subtract, ..Default::default()}),
            '*' => Some(Token {t_type: TokenType::Multiply, ..Default::default()}),
            '/' => Some(Token {t_type: TokenType::Divide, ..Default::default()}),
            '%' => Some(Token {t_type: TokenType::Modulus, ..Default::default()}),
            '^' => Some(Token {t_type: TokenType::Exponent, ..Default::default()}),
            '=' => Some(Token {t_type: TokenType::EqualSign, ..Default::default()}), // todo: equality
            '(' => Some(Token {t_type: TokenType::ParenOpen, ..Default::default()}),
            ')' => Some(Token {t_type: TokenType::ParenClose, ..Default::default()}),
            '{' => Some(Token {t_type: TokenType::BraceOpen, ..Default::default()}),
            '}' => Some(Token {t_type: TokenType::BraceClose, ..Default::default()}),
            _   => Some(Token {t_type: TokenType::Unknown, ..Default::default()})
        };

        if tok.is_some() {
            tokens.push(tok.unwrap());
        }
    }

    return tokens;
}

