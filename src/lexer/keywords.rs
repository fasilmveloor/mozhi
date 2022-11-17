use crate::lexer::tokens::TokenType;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Keywords {
    pub list: HashMap<String, TokenType>,
}

// Fancy macro for ease of adding new keywords
macro_rules! keywordize {
    ($( $words:expr => $func:expr ), *) => {{
        let mut list = HashMap::new();
        $( for &word in &($words) { list.insert(word.to_string(), $func); } )*
        list
    }};
}

impl Keywords {
    pub fn new() -> Self {
        let list = keywordize!(
            ["ചരം" ] => TokenType::Declaration,
            ["ഇൻപുട്"] => TokenType::InputString,
            ["ഇന്പുട്_നമ്പർ"] => TokenType::InputNumber,
            ["എഴുതുക", "പ്രദർശിപ്പിക്കുക"] => TokenType::Write,
            ["ശെരിയാണോ"] => TokenType::If,
            ["അല്ലേൽ", "അല്ലെങ്കിൽ"] => TokenType::Else,
            ["ആവർത്തിക്കുക"] => TokenType::Loop,
            ["വലുതാണെകിൽ", ] => TokenType::GreaterThan,
            ["തുല്യമല്ലേൽ"] => TokenType::NotEqual,
            ["ചെറുതാണെങ്കിൽ", ] => TokenType::LessThan,
            ["തുല്യമാണെങ്കിൽ"] => TokenType::EqualTo,
            ["ഉം"] => TokenType::Um,
            ["നെകാൾ"] => TokenType::Nekal,
            ["തിരികെ_അയക്കുക"] => TokenType::Return
        );

        Self { list }
    }
}
