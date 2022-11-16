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
    പ്രിന്റ് 'മൊഴിയിലേക്ക് സ്വാഗതം \n';
    ";
    let mut lex = Lexer::new(&code, HashMap::new(), 0);
    let parsed = parse(&code, &mut lex);

    println!("{:?}", parsed);
    let expected = SourceUnit(
        [
            Statement(Write((5, 93), StringLiteral((30, 53), TokenType::Literal(1))))
        ]
        .to_vec(),
    );
    assert_eq!(expected, parsed.unwrap());
}
