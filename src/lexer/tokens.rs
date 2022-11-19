use std::fmt::Formatter;
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TokenType {
    // Declaration,
    Write,
    Input,
    LeftBrace,
    RightBrace,
    If,
    Else,
    Loop,
    Assignment,
    Plus,
    Minus,
    Product,
    Divide,
    Modulo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    EqualTo,
    NotEqual,
    SemiColon,
    OpenParantheses,
    CloseParantheses,
    Um,
    Nekal,
    Do,
    Thavana,
    Ennu,
    Comma,
    AngleOpen,
    AngleClose,
    Literal(usize),
    Integer(i64),
    Float(f64),
    Symbol(usize),
    Return,
    Print,
    UntilGreaterThan,
    UntilLessThan,
    UntilGreaterThanOrEqual,
    UntilLessThanOrEqual,
    UntilEqualTo,
    UntilNotEqual,
    Until,
    Anenkil,
    
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Write => write!(f, "എഴുതുക"),
            TokenType::Input => write!(f, "ഇൻപുട്"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::If => write!(f, "ശെരിയാണോ"),
            TokenType::Else => write!(f, "അല്ലേൽ"),
            TokenType::Loop => write!(f, "ആവർത്തിക്കുക"),
            TokenType::Assignment => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Product => write!(f, "*"),
            TokenType::Divide => write!(f, "/"),
            TokenType::Modulo => write!(f, "%"),
            TokenType::GreaterThan => write!(f, "വലുതാണെകിൽ"),
            TokenType::LessThan => write!(f, "ചെറുതാണെങ്കിൽ"),
            TokenType::GreaterThanOrEqual => write!(f, "വലുതോതുല്യമോആണെങ്കിൽ"),
            TokenType::LessThanOrEqual => write!(f, "ചെറുതോതുല്യമോആണെങ്കിൽ"),
            TokenType::EqualTo => write!(f, "തുല്യമാണെങ്കിൽ"),
            TokenType::NotEqual => write!(f, "തുല്യമല്ലേൽ"),
            TokenType::SemiColon => write!(f, ";"),
            TokenType::OpenParantheses => write!(f, "("),
            TokenType::CloseParantheses => write!(f, ")"),
            TokenType::Um => write!(f, "ഉം"),
            TokenType::Nekal => write!(f, "നെകാൾ"),
            TokenType::Literal(literal) => write!(f, "{}", literal),
            TokenType::Integer(number) => write!(f, "{}", number),
            TokenType::Float(number) => write!(f, "{}", number),
            TokenType::Symbol(symbol) => write!(f, "{}", symbol),
            TokenType::Comma => write!(f, ","),
            TokenType::Return => write!(f, "തിരികെ_അയക്കുക"),
            TokenType::AngleOpen => write!(f, "<"),
            TokenType::AngleClose => write!(f, ">"),
            TokenType::Do => write!(f, "ചെയ്യുക"),
            TokenType::Ennu => write!(f, "എന്നു"),
            TokenType::Thavana => write!(f, "തവണ"),
            TokenType::Print => write!(f, "പ്രിന്റ്"),
            TokenType::UntilGreaterThan => write!(f, "വലുതാകുന്നതുവരെ"),
            TokenType::UntilLessThan => write!(f, "ചെറുതാകുന്നതുവരെ"),
            TokenType::UntilGreaterThanOrEqual => write!(f, "വലുതോതുല്യമോആകുന്നതുവരെ"),
            TokenType::UntilLessThanOrEqual => write!(f, "ചെറുതോതുല്യമോആകുന്നതുവരെ"),
            TokenType::UntilEqualTo => write!(f, "തുല്യമാകുന്നതുവരെ"),
            TokenType::UntilNotEqual => write!(f, "തുല്യമല്ലാതാകുന്നതുവരെ"),
            TokenType::Until => write!(f, "ആകുന്നതുവരെ"),
            TokenType::Anenkil => write!(f, "ആണെങ്കിൽ"),
        }
    }
}
