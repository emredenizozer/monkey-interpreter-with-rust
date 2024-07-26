use crate::token::Token;

struct Lexer {
    input: Vec<char>
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        todo!();
    }

    fn next_token(&self) -> Token {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use std::usize;

    use crate::token::{Token, TokenKind};

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input: &str = "=+(){},;";

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string()
            },
            Token {
                kind: TokenKind::PLUS,
                literal: "+".to_string()
            },
            Token {
                kind: TokenKind::LPARENTHESIS,
                literal: "(".to_string()
            },
            Token {
                kind: TokenKind::RPARENTHESIS,
                literal: ")".to_string()
            },
            Token {
                kind: TokenKind::LBRACE,
                literal: "{".to_string()
            },
            Token {
                kind: TokenKind::RBRACE,
                literal: "}".to_string()
            },
            Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string()
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string()
            },
            Token {
                kind: TokenKind::EOF,
                literal: "".to_string()
            }
        ];

        let lexer = Lexer::new(input);

        for (idx, expected_token) in expected.into_iter().enumerate() {
            let recv_token = lexer.next_token();
            assert_eq!(
                expected_token.kind, recv_token.kind,
                "tests[{idx}] - token type wrong. expected={}, got={}",
                expected_token.kind, recv_token.kind
            );
            assert_eq!(
                expected_token.literal, recv_token.literal,
                "tests[{idx}] - literal wrong. expected={}, got={}",
                expected_token.literal, recv_token.literal
            );
        }
    }
}