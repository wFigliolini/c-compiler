use std::fmt;
enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    StringLiteral(String),
    Operator(String),
    Separator(String),
}

impl Token {
    fn new(token_type: &str, value: String) -> Token {
        match token_type {
            "keyword" => Token::Keyword(value),
            "identifier" => Token::Identifier(value),
            "literal" => Token::Literal(value),
            "string_literal" => Token::StringLiteral(value),
            "operator" => Token::Operator(value),
            "separator" => Token::Separator(value),
            _ => panic!("Unknown token type"),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Keyword(a), Token::Keyword(b)) => a == b,
            (Token::Identifier(a), Token::Identifier(b)) => a == b,
            (Token::Literal(a), Token::Literal(b)) => a == b,
            (Token::StringLiteral(a), Token::StringLiteral(b)) => a == b,
            (Token::Operator(a), Token::Operator(b)) => a == b,
            (Token::Separator(a), Token::Separator(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword(value) => write!(f, "Keyword({})", value),
            Token::Identifier(value) => write!(f, "Identifier({})", value),
            Token::Literal(value) => write!(f, "Literal({})", value),
            Token::StringLiteral(value) => write!(f, "StringLiteral({})", value),
            Token::Operator(value) => write!(f, "Operator({})", value),
            Token::Separator(value) => write!(f, "Separator({})", value),
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "int main() { return 0; }";
        let tokens = tokenize(input);
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0], Token::Keyword("int".to_string()));
        assert_eq!(tokens[1], Token::Identifier("main".to_string()));
        assert_eq!(tokens[2], Token::Separator("(".to_string()));
        assert_eq!(tokens[3], Token::Separator(")".to_string()));
        assert_eq!(tokens[4], Token::Separator("{".to_string()));
        assert_eq!(tokens[5], Token::Keyword("return".to_string()));
        assert_eq!(tokens[6], Token::Literal("0".to_string()));
        assert_eq!(tokens[7], Token::Separator(";".to_string()));
        assert_eq!(tokens[8], Token::Separator("}".to_string()));
    }
}
