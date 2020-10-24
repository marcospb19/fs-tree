use logos::Logos;

#[rustfmt::skip]
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Groups
    // Examples:
    //   - [group_name]
    //   -[spacing_is_optional]
    #[regex(" *- *\\[ *[^\\] ]+ *\\]", |lex| {
        let start = lex.slice().find("[").unwrap();
        let end = lex.slice().rfind("]").unwrap();

        let slice: &str = &lex.slice()[start + 1..end].trim();

        String::from(slice)
    })]
    Group(String),

    // Flags, escaped like strings, but use () instead of ""
    // Examples:
    //   (a, b)
    //   (unix)
    // #[regex(r"\(([^\)\\]|\\t|\\u|\\n|\\\))*\)", |lex| {
    //     let vec = vec![];
    //     let potential_flags =

    //     vec![]
    //     // lex.source()[lex.span()].chars().map(String::from).collect()
    // })]
    // Flags(Vec<String>),

    // Value token delimited by ""
    #[regex("\"[^\"]+\"", |lex| {
        let start = lex.slice().find("\"").unwrap();
        let end = lex.slice().rfind("\"").unwrap();

        let slice: &str = &lex.slice()[start + 1..end];

        String::from(slice)
    })]
    Value(String),

    #[token(":")]
    DoubleDots,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token("->")]
    SymlinkArrow,

    // New line or comma
    #[regex("(\\n|,)")]
    Separator,

    // Ignore whitespace
    #[regex(" \\t", logos::skip)]

    // // // Ignore whitespace
    // #[regex(r"[ \t\n\f]+", logos::skip)]
    // // Ignore comments, they start with two slashes
    // #[regex(r"//[^\n]*\n", logos::skip)]
    // // Anything unexpected
    #[error]
    Error,
}

#[cfg(test)]
mod lexer_tests {
    use super::{
        Token::{self, *},
        *,
    };

    // Test lexer
    fn tl(text: &str, expected: Token) {
        let mut lex = Token::lexer(text);
        assert_eq!(lex.next().unwrap(), expected)
    }

    #[test]
    fn group_regex() {
        tl("- [asd]", Group(String::from("asd")));
        tl("   -    [asd]", Group(String::from("asd")));
        tl("   -[ asd  ]", Group(String::from("asd")));
    }

    #[test]
    fn separator_regex() {
        tl("\n", Separator);
        tl(",", Separator);
    }
}
