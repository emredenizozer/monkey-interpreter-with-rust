use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPARENTHESIS,
    RPARENTHESIS,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::ILLEGAL => write!(f, "Illegal"),
            TokenKind::EOF => write!(f, "Eof"),
            TokenKind::IDENT => write!(f, "Ident"),
            TokenKind::INT => write!(f, "Int"),
            TokenKind::ASSIGN => write!(f, "="),
            TokenKind::PLUS => write!(f, "+"),
            TokenKind::COMMA => write!(f, ","),
            TokenKind::SEMICOLON => write!(f, ";"),
            TokenKind::LPARENTHESIS => write!(f, "("),
            TokenKind::RPARENTHESIS => write!(f, ")"),
            TokenKind::LBRACE => write!(f, "{{"),
            TokenKind::RBRACE => write!(f, "}}"),
            TokenKind::FUNCTION => write!(f, "Function"),
            TokenKind::LET => write!(f, "Let")
        }
    }
}