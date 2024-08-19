pub mod input_handler {

    use std::io;
    use crate::simplifier::expression::factor::Factor;
    use super::super::simplify::simplifier_module as simplifier;
    use super::super::parse::{lexer, parser};

    pub fn read_input() {
        println!("Enter the expression you want to simplify: ");
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        simplifier::presimplify(&mut expression);
        let mut lexer = lexer::lexer_module::Lexer::new(&expression);
        lexer.next();
        let mut parser = parser::parser_module::Parser::new(lexer);
        let expr = parser.parse_expr();
        let mut poly = expr.to_polynomial().build_string();
        simplifier::postsimplify(&mut poly);
        println!("Simplified expression: {}", poly);
    }

}