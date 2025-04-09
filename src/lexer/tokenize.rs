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
    let mut current_token = String::new();
    for c in input.chars() {
        if c.is_whitespace() {
            if !current_token.is_empty() {
                match process_current_token(&mut current_token, &mut tokens) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            continue;
        }
        if c.is_alphanumeric() || c == '_' || c == '.' {
            current_token.push(c);
        } else if c == '"' {
            //String literal found, start subloop until closing quote is found
            current_token.push(c);
            while let Some(next_c) = input.chars().next() {
                current_token.push(next_c);
                if next_c == '"' {
                    break;
                }
            }
        } else if c == ';' || c == '{' || c == '}' || c == '(' || c == ')' {
            if !current_token.is_empty() {
                match process_current_token(&mut current_token, &mut tokens) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            tokens.push(Token::new("separator", c.to_string()));
        } else {
            return Err("Unknown token");
        }
    }
    Ok(tokens)
}

fn process_current_token(
    current_token: &mut String,
    tokens: &mut Vec<Token>,
) -> Result<(), &'static str> {
    if is_keyword(current_token) {
        tokens.push(Token::new("keyword", current_token.clone()));
    } else if is_identifier(current_token) {
        tokens.push(Token::new("identifier", current_token.clone()));
    } else if is_literal(current_token) {
        tokens.push(Token::new("literal", current_token.clone()));
    } else if is_string_literal(current_token) {
        tokens.push(Token::new("string_literal", current_token.clone()));
    } else {
        return Err("Unknown token");
    }
    current_token.clear();
    Ok(())
}

fn is_keyword(token: &str) -> bool {
    // full list of C keywords from R&K C book
    let keywords = [
        "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
        "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return",
        "short", "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned",
        "void", "volatile", "while",
    ];
    keywords.contains(&token)
}

fn is_identifier(token: &str) -> bool {
    // Identifiers must start with a letter or underscore, followed by letters, digits, or underscores
    let first_char = token.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }
    for c in token.chars().skip(1) {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }
    true
}

fn is_literal(token: &str) -> bool {
    if token.parse::<f64>().is_ok() {
        return true;
    }

    false
}

fn is_string_literal(token: &str) -> bool {
    token.starts_with('"') && token.ends_with('"')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new_keyword() {
        let token = Token::new("keyword", "int".to_string());
        assert_eq!(token, Token::Keyword("int".to_string()));
    }

    #[test]
    fn test_token_new_identifier() {
        let token = Token::new("identifier", "foo".to_string());
        assert_eq!(token, Token::Identifier("foo".to_string()));
    }

    #[test]
    fn test_token_new_literal() {
        let token = Token::new("literal", "123".to_string());
        assert_eq!(token, Token::Literal("123".to_string()));
    }

    #[test]
    fn test_token_new_string_literal() {
        let token = Token::new("string_literal", "\"hello\"".to_string());
        assert_eq!(token, Token::StringLiteral("\"hello\"".to_string()));
    }
    #[test]
    fn test_token_new_operator() {
        let token = Token::new("operator", "+".to_string());
        assert_eq!(token, Token::Operator("+".to_string()));
    }

    #[test]
    fn test_token_new_separator() {
        let token = Token::new("separator", ";".to_string());
        assert_eq!(token, Token::Separator(";".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_invalid_token() {
        let _result = Token::new("invalid", "foo".to_string());
    }

    #[test]
    #[should_panic]
    fn test_invalid_token_comparison() {
        let token1 = Token::new("keyword", "int".to_string());
        let token2 = Token::new("identifier", "foo".to_string());
        assert_eq!(token1, token2);
    }

    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("int"));
        assert!(is_keyword("return"));
        assert!(!is_keyword("foo"));
    }
    #[test]
    fn test_is_identifier() {
        assert!(is_identifier("foo"));
        assert!(is_identifier("_foo"));
        assert!(is_identifier("bar_123"));
        assert!(!is_identifier("foo&bar"));
        assert!(!is_identifier("123foo"));
    }

    #[test]
    fn test_is_literal() {
        assert!(is_literal("123"));
        assert!(is_literal("0.5"));
        assert!(!is_literal("\"hello\""));
        assert!(!is_literal("foo"));
        assert!(!is_literal("123foo"));
    }

    #[test]
    fn test_is_string_literal() {
        assert!(is_string_literal("\"hello\""));
        assert!(!is_string_literal("hello"));
        assert!(!is_string_literal("\"hello"));
        assert!(!is_string_literal("hello\""));
    }

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
