use std::ops;

use logos::Logos;

pub(crate) type SpannedLexToken = (LexToken, ops::Range<usize>);

#[derive(Logos, Debug, PartialEq)]
pub enum LexToken {
    // TODO: check if escaped "\]" is being supported
    // Groups
    // Examples:
    //   - [group_name]
    //   -[spacing_is_optional]
    #[regex(" *- *\\[ *[^\\] ]+ *\\]", |lex| {
        // Safe unwraps
        let start = lex.slice().find('[').unwrap();
        let end = lex.slice().rfind(']').unwrap();

        let slice: &str = &lex.slice()[start + 1..end].trim();
        String::from(slice)
    })]
    Group(String),

    // TODO: support escaped close parenthesis "\)"
    // Tags, escaped like strings, but use () instead of ""
    // Examples:
    //   (a, b)
    //   (unix)
    #[regex(r"\(([^\)\\]|\\t|\\u|\\n|\\\))*\)", |lex|
        let span = lex.span();
        let slice = &lex.source()[span.start + 1 .. span.end - 1]; // Without ()
        slice.split(',').map(|x| x.trim().to_string()).collect::<Vec<String>>()
    )]
    Tags(Vec<String>),

    // TODO: support escaped double quotes "\""
    // Value token delimited by ""
    #[regex("\"[^\"]+\"", |lex| {
        let start = lex.slice().find('\"').unwrap();
        let end = lex.slice().rfind('\"').unwrap();

        lex.slice()[start + 1..end].to_string()
    })]
    Value(String),

    #[token(":")]
    DoubleDots,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[regex(r"-?>")]
    SymlinkArrow,

    // New line or comma separators
    #[regex(r"(\n|,)" , |lex| {
        // Extract what char it was
        lex.slice().chars().next().unwrap()
    })]
    Separator(char),

    // Ignore whitespace
    #[regex(r" *", logos::skip)]
    // Ignore tab
    #[regex(r"\t+", logos::skip)]
    // TODO: switch to # comments
    // Ignore comments, they start with two slashes
    #[regex(r"//[^\n]*", logos::skip)]
    // Anything unexpected
    #[error]
    LexError,
}

pub fn run_lexer(input_text: &str) -> Vec<SpannedLexToken> {
    LexToken::lexer(input_text).spanned().collect()
}

#[cfg(test)]
mod lexer_tests {
    use super::{
        LexToken::{self, *},
        *,
    };

    // Test lexer
    fn test(text: &str, expected: LexToken) {
        let mut lex = LexToken::lexer(text);
        assert_eq!(lex.next().unwrap(), expected);
    }

    #[test]
    fn group_regex() {
        test("- [asd]", Group(String::from("asd")));
        test("   -    [asd]", Group(String::from("asd")));
        test("   -[ asd  ]", Group(String::from("asd")));
    }

    #[test]
    fn separator_regex() {
        test("\n", Separator('\n'));
        test(",", Separator(','));
    }

    #[test]
    fn no_errors_check() {
        let files = ["examples/simplest.tree", "examples/simple.tree", "examples/dotao.tree"];

        for file in &files {
            let text = std::fs::read_to_string(file).unwrap();
            let mut lex = LexToken::lexer(&text);
            while let Some(token) = lex.next() {
                assert!(!matches!(token, LexToken::LexError));
            }
        }
    }
}
