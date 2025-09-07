use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("fn")]
    KwFn,
    #[token("let")]
    KwLet,
    #[token("use")]
    KwUse,
    #[token("rail")]
    KwRail,
    #[token("on")]
    KwOn,
    #[token("success")]
    KwSuccess,
    #[token("error")]
    KwErrorKw,
    #[token("print")]
    KwPrint,

    #[token("|>")]
    Pipe,
    #[token(":")]
    Colon,
    #[token("=")]
    Eq,
    #[token("+")]
    Plus,
    #[token("/")]
    Slash,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    Str(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Int(i64),

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"\r?\n", priority = 10)]
    Newline,
}

pub fn lexer(source: &str) -> Lexer<Token> {
    Token::lexer(source)
}

pub fn lex(source: &str) -> Vec<Token> {
    lexer(source).map(Result::ok).flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_short_fn_sum() {
        let src = "fn sum a b: a+b";
        let toks = lex(src);

        assert_eq!(
            toks,
            vec![
                Token::KwFn,
                Token::Ident("sum".into()),
                Token::Ident("a".into()),
                Token::Ident("b".into()),
                Token::Colon,
                Token::Ident("a".into()),
                Token::Plus,
                Token::Ident("b".into()),
            ]
        );
    }

    #[test]
    fn lexes_formal_fn_sum() {
        let src = "fn sum (a int) (b int) : int\n\ta+b";
        let toks = lex(src);

        assert_eq!(
            toks,
            vec![
                Token::KwFn,
                Token::Ident("sum".into()),
                Token::LParen,
                Token::Ident("a".into()),
                Token::Ident("int".into()),
                Token::RParen,
                Token::LParen,
                Token::Ident("b".into()),
                Token::Ident("int".into()),
                Token::RParen,
                Token::Colon,
                Token::Ident("int".into()),
                Token::Newline,
                Token::Ident("a".into()),
                Token::Plus,
                Token::Ident("b".into()),
            ]
        );
    }

    #[test]
    fn lexes_pipeline_and_print_and_string() {
        let src = r#"
use std
rail:
  sum 10 11
  div 20
on success value : print f"success {e}"
on error value : print f"error {e}"
"#;

        let toks = lex(src);

        assert!(toks.contains(&Token::KwUse));
        assert!(toks.contains(&Token::KwRail));
        assert!(toks.contains(&Token::KwOn));
        assert!(toks.contains(&Token::KwSuccess));
        assert!(toks.contains(&Token::KwErrorKw));
        assert!(toks.contains(&Token::KwPrint));
        assert!(
            toks.iter()
                .any(|t| matches!(t, Token::Str(s) if s.starts_with("success ")))
        );
        assert!(
            toks.iter()
                .any(|t| matches!(t, Token::Str(s) if s.starts_with("error ")))
        );
    }
}
