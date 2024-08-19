pub mod input_handler {

    use std::io;
    use super::super::simplify::simplifier_module as simplifier;

    pub fn read_input() {
        println!("Enter the expression you want to simplify: ");
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        simplifier::presimplify(&mut expression);
        println!("Simplified expression: {}", expression);
    }

}