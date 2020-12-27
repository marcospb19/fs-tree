use logos::Logos;

use std::ops;

pub(crate) type SpannedLexToken = (LexToken, ops::Range<usize>);

#[derive(Logos, Debug, PartialEq)]
pub enum LexToken {
    // Groups
    // Examples:
    //   - [group_name]
    //   -[spacing_is_optional]
    #[regex(" *- *\\[ *[^\\] ]+ *\\]", |lex| {
        let start = lex.slice().find('[').unwrap();
        let end = lex.slice().rfind(']').unwrap();

        let slice: &str = &lex.slice()[start + 1..end].trim();

        String::from(slice)
    })]
    Group(String),

    // Flags, escaped like strings, but use () instead of ""
    // Examples:
    //   (a, b)
    //   (unix)
    #[regex(r"\(([^\)\\]|\\t|\\u|\\n|\\\))*\)", |_lex| {
        // let vec = vec![];
        // let potential_flags = vec![];
        // let vec: Vec<String> = lex.source()[lex.span()].chars().map(String::from).collect();
        // vec
        vec![]
    })]
    Flags(Vec<String>),

    // Value token delimited by ""
    #[regex("\"[^\"]+\"", |lex| {
        let start = lex.slice().find('\"').unwrap();
        let end = lex.slice().rfind('\"').unwrap();

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
    #[regex("(\\n|,)" , |lex| {
        let c = lex.slice().chars().next().unwrap();
        if c != '\n' && c != ',' {
            unreachable!()
        }
        c
    })]
    Separator(char),
    // // // Ignore whitespace
    // #[regex(r"[ \t\n\f]+", logos::skip)]

    // Ignore whitespace
    #[regex("( |\\t)+", logos::skip)]
    // Ignore comments, they start with two slashes
    #[regex(r"//[^\n]*\n", logos::skip)]
    // // Anything unexpected
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
    fn tl(text: &str, expected: LexToken) {
        let mut lex = LexToken::lexer(text);
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
        tl("\n", Separator('\n'));
        tl(",", Separator(','));
    }

    #[test]
    fn no_errors_check() {
        let files = [
            "examples/simplest.tree",
            "examples/simple.tree",
            // "examples/dotao.tree", // Need flags feature
        ];

        let mut should_panic = false;

        for file in &files {
            let text = std::fs::read_to_string(file).unwrap();
            let mut lex = LexToken::lexer(&text);
            while let Some(token) = lex.next() {
                if matches!(token, LexToken::LexError) {
                    should_panic = true;
                }
            }
        }

        if should_panic {
            panic!();
        }
    }
}
