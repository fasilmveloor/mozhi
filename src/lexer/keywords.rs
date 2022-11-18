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
            ["ഇൻപുട്"] => TokenType::Input,
            ["എഴുതുക", "പ്രദർശിപ്പിക്കുക", "കാണിക്കുക"] => TokenType::Write,
            ["ശെരിയാണോ"] => TokenType::If,
            ["അല്ലേൽ", "അല്ലെങ്കിൽ"] => TokenType::Else,
            ["ആവർത്തിക്കുക"] => TokenType::Loop,
            ["വലുതാണെകിൽ", ] => TokenType::GreaterThan,
            ["തുല്യമല്ലേൽ", "തുല്യമല്ലെങ്കിൽ"] => TokenType::NotEqual,
            ["ചെറുതാണെങ്കിൽ", ] => TokenType::LessThan,
            ["തുല്യമാണെങ്കിൽ"] => TokenType::EqualTo,
            ["ഉം"] => TokenType::Um,
            ["നെകാൾ"] => TokenType::Nekal,
            ["തിരികെ_അയക്കുക"] => TokenType::Return,
            ["ചെയ്യുക"] => TokenType::Do,
            ["എന്നു"] => TokenType::Ennu,
            ["തവണ"] => TokenType::Thavana,
            ["പ്രിന്റ്"] => TokenType::Print,
            ["തുല്യമാകുന്നത്_വരെ"] => TokenType::UntilEqualTo,
            ["തുല്യമല്ലാതാകുന്നത്_വരെ"] => TokenType::UntilNotEqual,
            ["ചെറുതാകുന്നത്_വരെ"] => TokenType::UntilLessThan,
            ["വലുതാകുന്നത്_വരെ"] => TokenType::UntilGreaterThan,
            ["വലുതോതുല്യമോആകുന്നത്_വരെ"] => TokenType::UntilGreaterThanOrEqual,
            ["ചെറുതോതുല്യമോആകുന്നത്_വരെ"] => TokenType::UntilLessThanOrEqual
            //["പരിഹരിക്കുക"] => TokenType::Break
        );

        Self { list }
    }
}
