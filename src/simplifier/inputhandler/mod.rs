pub mod input_handler {

    use std::io;
    use super::super::simplify::simplifier_module as simplifier;
    use super::super::parse::{lexer, parser};

    pub fn read_input() {
        println!("Enter the expression you want to simplify: ");
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        simplifier::presimplify(&mut expression);
        //let lexer = lexer::lexer_module::Lexer::new(&expression);
        //let mut parser = parser::parser_module::Parser::new(lexer);
        println!("Simplified expression: {}", expression);
    }

}