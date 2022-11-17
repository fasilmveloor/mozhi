use crate::executor::ast::Expression::*;
use crate::executor::ast::SourceUnitPart::Statement;
use crate::executor::ast::Statement::*;
use crate::executor::ast::*;
use crate::lexer::tokens::TokenType;

use crate::lexer::Lexer;
use crate::parser::parse;
use std::collections::HashMap;

#[test]
fn parser_test() {
    let code = "
    ചരം x;
    x = ഇൻപുട്_നമ്പർ;
    ";
    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);

    println!("{:?}", parsed);
    let expected = SourceUnit(
        [
            Statement(Declaration((5, 17), Symbol((15, 15), TokenType::Symbol(1)))),
            Statement(Assignment(
                (22, 61),
                Symbol((22, 22), TokenType::Symbol(1)), 
                Symbol((26, 37), TokenType::Symbol(2)),
            )),
        ]
        .to_vec(),
    );
    assert_eq!(expected, parsed.unwrap());
}
