pub mod token_type {

    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        Num,
        X,
        Multi,
        Exp,
        Add,
        Sub,
        Lp,
        Rp,
        Null,
    }
}

pub mod lexer_module {

    use super::token_type::TokenType;

    pub struct Lexer {
        pos: i32,
        expr: String,
        token_type: TokenType,
        token_number: String,
        tokens: Vec<String>,
        token_types: Vec<TokenType>,
    }

    impl Lexer {
        pub fn new(expr: &str) -> Lexer {
            Lexer {
                pos: 0,
                expr: expr.to_string(),
                token_type: TokenType::Null,
                token_number: "".to_string(),
                tokens: Vec::new(),
                token_types: Vec::new(),
            }
        }

        pub fn get_number(&mut self) -> String {
            let mut number = "".to_string();
            while self.pos < self.expr.len().try_into().unwrap() {
                let ch = self.expr.chars().nth(self.pos as usize).unwrap();
                if ch.is_ascii_digit() {
                    number.push(ch);
                    self.pos += 1;
                } else {
                    break;
                }
            }
            number
        }

        pub fn next(&mut self) {
            if self.pos == self.expr.len().try_into().unwrap() {
                return;
            }
            let ch = self.expr.chars().nth(self.pos as usize).unwrap();
            if ch.is_ascii_digit() {
                self.token_type = TokenType::Num;
                self.token_number = self.get_number();
                self.tokens.push(self.token_number.clone());
                self.token_types.push(TokenType::Num);
            } else {
                match ch {
                    '+' => {
                        self.pos += 1;
                        self.token_type = TokenType::Add;
                        self.tokens.push("+".to_string());
                        self.token_types.push(TokenType::Add);
                    }
                    '-' => {
                        self.pos += 1;
                        self.token_type = TokenType::Sub;
                        self.tokens.push("-".to_string());
                        self.token_types.push(TokenType::Sub);
                    }
                    '*' => {
                        self.pos += 1;
                        self.token_type = TokenType::Multi;
                        self.tokens.push("*".to_string());
                        self.token_types.push(TokenType::Multi);
                    }
                    '^' => {
                        self.pos += 1;
                        self.token_type = TokenType::Exp;
                        self.tokens.push("^".to_string());
                        self.token_types.push(TokenType::Exp);
                    }
                    '(' => {
                        self.pos += 1;
                        self.token_type = TokenType::Lp;
                        self.tokens.push("(".to_string());
                        self.token_types.push(TokenType::Lp);
                    }
                    ')' => {
                        self.pos += 1;
                        self.token_type = TokenType::Rp;
                        self.tokens.push(")".to_string());
                        self.token_types.push(TokenType::Rp);
                    }
                    'x' => {
                        self.pos += 1;
                        self.token_type = TokenType::X;
                        self.tokens.push("x".to_string());
                        self.token_types.push(TokenType::X);
                    }
                    _ => {}
                }
            }
        }

        pub fn get_cur_token(&self) -> &TokenType {
            &self.token_type
        }

        pub fn get_token_number(&self) -> &String {
            &self.token_number
        }

    }

    #[cfg(test)]
    mod lexer_test {
        #[test] 
        fn test() {
            let expr = String::from("20+300*x^2");
            let mut lexer = super::Lexer::new(&expr);
            while (lexer.pos as usize) < expr.len() {
                lexer.next();
                println!("{:?}", lexer.token_type);
            }
        }
    }

}