pub mod parser_module {

    use core::panic;
    use super::super::lexer::lexer_module as lexer;
    use super::super::lexer::token_type::TokenType;
    use super::super::super::expression::pow::Pow;
    use super::super::super::expression::expr::Expr;
    use super::super::super::expression::term::Term;
    use super::super::super::expression::factor::Factor;
    use super::super::super::expression::number::Number;

    pub struct Parser {
        lexer: lexer::Lexer,
    }

    impl Parser {

        pub fn new(lexer: lexer::Lexer) -> Parser {
            Parser {
                lexer,
            }
        }

        pub fn parse_expr<T: Factor>(&mut self) -> Expr<T>{
            let mut expr:Expr<T>  = Expr::new();
            let mut sign = 1;
            if *(self.lexer.get_cur_token()) == TokenType::Sub {
                self.lexer.next();
                sign = -1; 
            } else if *(self.lexer.get_cur_token()) == TokenType::Add {
                self.lexer.next();
            }
            expr.add_term(self.parse_term(sign));
            while *(self.lexer.get_cur_token()) == TokenType::Add || *(self.lexer.get_cur_token()) == TokenType::Sub 
            {
                let sign = if *(self.lexer.get_cur_token()) == TokenType::Sub { -1 } else { 1 };
                self.lexer.next();
                expr.add_term(self.parse_term(sign));
            }
            expr
        }

        pub fn parse_term<T: Factor> (&mut self, sign: i32) -> Term<T> {
            let mut term = Term::new(sign);
            term.add_factor(self.parse_factor());
            while *(self.lexer.get_cur_token()) == TokenType::Multi {
                self.lexer.next();
                term.add_factor(self.parse_factor());
            }
            term
        }

        pub fn parse_factor<T: Factor> (&mut self) -> T {
            let token = *(self.lexer.get_cur_token());
            if token == TokenType::Lp {
                self.lexer.next();
                let expr = self.parse_expr();
                self.lexer.next();
                if *(self.lexer.get_cur_token()) == TokenType::Exp {
                    self.lexer.next();
                    let pow = self.lexer.get_number().parse::<i32>().unwrap();
                    expr.set_pow(pow);
                    self.lexer.next();
                }
                return expr;
            } else if token == TokenType::Num {
                let number = self.lexer.get_number().parse::<i32>().unwrap();
                self.lexer.next();
                return Number::new(number);
            } else if token == TokenType::Sub {
                self.lexer.next();
                let number = self.lexer.get_number().parse::<i32>().unwrap();
                self.lexer.next();
                return Number::new(-number);
            } else if token == TokenType::X {
                self.lexer.next();
                if *(self.lexer.get_cur_token()) == TokenType::Exp {
                    self.lexer.next();
                    let pow = self.lexer.get_number().parse::<i32>().unwrap();
                    self.lexer.next();
                    return Pow::new(pow);
                } else {
                    return Pow::new(1);
                }
            } else {
                panic!("Invalid token");
            }
        }

    }
}