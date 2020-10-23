use logos::Logos;

#[rustfmt::skip]
#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken {
    // // Groups
    // // Examples:
    // //   - [group_name]
    // //   -[spacing_is_optional]
    // #[regex(r"- *\[\[([^\]\\]|\\t|\\u|\\\])*\]\]")]
    // Group(String),

    // // Flags, escaped like strings, but use () instead of ""
    // // Examples:
    // //   (a, b)
    // //   (unix)
    // #[regex(r"\(([^\)\\]|\\t|\\u|\\n|\\\))*\)", |lex| {
    //     let vec = vec![];
    //     let potential_flags =

    //     vec![]
    //     // lex.source()[lex.span()].chars().map(String::from).collect()
    // })]
    // Flags(Vec<String>),

    // // Value token delimited by ""
    // #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    // Value(String),

    // #[regex(":")]
    // DoubleDots,

    // #[regex(r"\[")]
    // OpenBracket,

    // #[regex("]")]
    // CloseBracket,

    // #[regex("->")]
    // SymlinkArrow,

    // #[token(r"(,|\\n)")]
    // Separator,

    // // // Ignore whitespace
    // #[regex(r"[ \t\n\f]+", logos::skip)]
    // // Ignore comments, they start with two slashes
    // #[regex(r"//[^\n]*\n", logos::skip)]
    // // Anything unexpected
    #[error]
    Error,
}
