use logos::Logos;

use std::ops;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // A group is escaped like a string, but using () instead
    #[regex(r"\(([^\)\\]|\\t|\\u|\\n|\\\))*\)")]
    Group,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,

    #[regex(":")]
    DoubleDots,

    #[regex(r"\[")]
    OpenBracket,

    // #[regex(r"->")]
    // SymlinkArrow,
    #[regex("]")]
    CloseBracket,

    #[token(",")]
    Comma,

    // Ignore whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    // Ignore comments, they start with two slashes
    #[regex(r"//[^\n]*\n", logos::skip)]
    // Anything unexpected
    #[error]
    Error,
}

type Range = ops::Range<usize>;

pub fn text_as_tokens(text: impl AsRef<str>) -> Vec<(Token, Range)> {
    let mut lex = Token::lexer(&text.as_ref());

    // This vec contains Tokens and the ranges of their text in the text argument
    let mut vec: Vec<(Token, Range)> = vec![];

    // None marks the end
    while let Some(token) = lex.next() {
        match token {
            // If found an error, imemdiately exit reporting it's location
            Token::Error => return vec![(token, lex.span())],

            // For Group and String, remove "" and ()
            // That is, the returned Range refers to the text delimited by "", without including ""
            // "\"Lorem Ipsum\"" -> "Lorem Ipsum"
            // "(Lorem Ipsum)"   -> "Lorem Ipsum"
            Token::Group | Token::String => {
                let mut range = lex.span();
                range.start += 1;
                range.end -= 1;
                vec.push((token, range));
            },

            // The rest, add to the vector without any mutation
            Token::DoubleDots | Token::OpenBracket | Token::CloseBracket | Token::Comma => {
                vec.push((token, lex.span()));
            },
        }
    }

    // Return the vec with tokens and ranges, our lexer stage here is done, now the
    // parser module will take care of this information
    vec
}
