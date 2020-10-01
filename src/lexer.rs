use logos::Logos;

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
    //
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

// pub fn parse_tree_from_file_logos(_file_path: &str) {
//     let text = fs::read_to_string("/home/marcospb19/dotao.tree").expect("dotao.tree expected.");
//     let mut lex = Token::lexer(&text);

//     let mut vec: Vec<&str> = vec![];

//     loop {
//         let next = lex.next();

//         if let None = next {
//             break;
//         }

//         let range = lex.span();
//         let range = &text[range];
//         vec.push(range);
//         // let range = next.slice();
//         // println!("{:#?}  ", range);

//         // if let Some(Token::Error) = next {
//         //     eprintln!("error!");
//         //     break;
//         // }

//         // println!("{:#?}", lex.slice());
//     }

//     println!("{:#?}", vec);
// }

// pub fn parse_tree_from_file(file_path: &str) {
//     let string: Vec<char> = fs::read_to_string(file_path)
//         .expect("Couldn't read file in parse_tree function.")
//         .chars()
//         .collect();

//     let tokens = tokenize_from_string(string);

//     if let Err(e) = tokens {
//         println!("Rolou um erro");
//         println!("{:?}", e);
//         return;
//     }

//     let tokens = tokens.unwrap();

//     for token in &tokens {
//         if let Token {
//             token_type: TokenType::NewLine,
//             ..
//         } = token
//         {
//             print!(" \\n");
//         } else {
//             print!("\n{:?}", token);
//         }
//     }
//     println!();

//     let file_list = process_tokenized(tokens);
//     println!("file list:\n");

//     for file in file_list {
//         println!("{}", file);
//     }
// }

// fn process_tokenized(_tokens: Vec<Token>) -> Vec<String> {
//     let v = Vec::<String>::new();

//     // state...

//     for _token in _tokens {}
//     v
// }

// #[derive(Debug)]
// enum LexerState {
//     Normal,
//     ReadingStringValue,
//     ReadingGroupValue,
//     IgnoringComment,
// }

// #[derive(Debug)]
// enum TokenType {
//     Group { value: String }, // (value)
//     File { value: String },  // "value"
//     Comma,                   // ,
//     Hyphen,                  // -
//     NewLine,                 // \n
//     DoubleDots,              // :
//     OpenBracket,             // [
//     CloseBracket,            // ]
// }

// #[derive(Debug)]
// struct Token {
//     position: Position2D,
//     token_type: TokenType,
// }

// #[derive(Debug, Clone)]
// struct Position2D {
//     x: u64,
//     y: u64,
// }

// #[derive(Debug)]
// enum LexerError {
//     UnexpectedCharacter { position: Position2D, value: String },
//     UnfinishedParse { token_type: LexerState },
// }

// fn tokenize_from_string(mut string: Vec<char>) -> Result<Vec<Token>, LexerError> {
//     // Add a space at the begin and end to allow safe access
//     string.insert(0, ' ');
//     string.push(' ');

//     let mut tokens = Vec::<Token>::new();
//     let mut token_value = String::new();
//     let mut state = LexerState::Normal;
//     let mut pos = Position2D { x: 0, y: 0 }; // Position2D of characters in string

//     // Lexer loop to fill `tokens: Vec<Token>`
//     //
//     // Safe access to the string's [i], [i-1] and [i+1]
//     for i in 1..string.len() - 1 {
//         // Some aliases to keep excessive checks shorter and clearer
//         // c is the current char
//         let c = string[i];
//         let c_previous = string[i - 1];
//         let c_next = string[i + 1];

//         // Update position
//         if c_previous == '\n' {
//             pos.x = 0;
//             pos.y += 1;
//         } else {
//             pos.x += 1;
//         }

//         // Lexer machine state
//         match state {
//             LexerState::Normal => {
//                 // State machine booleans
//                 let is_start_of_comment = c == '/' && c_next == '/' || c == '#';
//                 let is_start_of_string = c == '\"';
//                 let is_start_of_group = c == '(';

//                 if is_start_of_comment {
//                     state = LexerState::IgnoringComment;
//                     continue;
//                 } else if is_start_of_string {
//                     state = LexerState::ReadingStringValue;
//                     continue;
//                 } else if is_start_of_group {
//                     state = LexerState::ReadingGroupValue;
//                     continue;
//                 }

//                 let should_skip_iteration = c == ' ' || c == '\t' || c == '\r';
//                 if should_skip_iteration {
//                     continue;
//                 }

//                 match c {
//                     '[' => tokens.push(Token {
//                         token_type: TokenType::OpenBracket,
//                         position: pos.clone(),
//                     }),
//                     ']' => tokens.push(Token {
//                         token_type: TokenType::CloseBracket,
//                         position: pos.clone(),
//                     }),
//                     ',' => tokens.push(Token {
//                         token_type: TokenType::Comma,
//                         position: pos.clone(),
//                     }),
//                     ':' => tokens.push(Token {
//                         token_type: TokenType::DoubleDots,
//                         position: pos.clone(),
//                     }),
//                     '-' => tokens.push(Token {
//                         token_type: TokenType::Hyphen,
//                         position: pos.clone(),
//                     }),
//                     '\n' => tokens.push(Token {
//                         token_type: TokenType::NewLine,
//                         position: pos.clone(),
//                     }),

//                     // else
//                     _ => {
//                         return Err(LexerError::UnexpectedCharacter {
//                             position: pos,
//                             value: c.to_string(),
//                         })
//                     }
//                 }
//             }
//             LexerState::ReadingStringValue => {
//                 let is_end_of_string = c == '\"' && c_previous != '\\';

//                 if is_end_of_string {
//                     tokens.push(Token {
//                         token_type: TokenType::File { value: token_value },
//                         position: pos.clone(),
//                     });
//                     // Reseting state
//                     token_value = "".into();
//                     state = LexerState::Normal;
//                 } else {
//                     token_value.push(c);
//                 }
//             }
//             LexerState::ReadingGroupValue => {
//                 let is_end_of_group = c == ')' && c_previous != '\\';

//                 if is_end_of_group {
//                     tokens.push(Token {
//                         token_type: TokenType::Group { value: token_value },
//                         position: pos.clone(),
//                     });
//                     // Reseting state
//                     token_value = "".into();
//                     state = LexerState::Normal;
//                 } else {
//                     token_value.push(c);
//                 }
//             }
//             LexerState::IgnoringComment => {
//                 if c == '\n' {
//                     state = LexerState::Normal;
//                 }
//             }
//         } // match
//     } // for

//     // If state is LexerState::Normal, success
//     if let LexerState::Normal = state {
//         Ok(tokens)
//     }
//     // Else, we have an error
//     else {
//         Err(LexerError::UnfinishedParse { token_type: state })
//     }
// }

// #[cfg(test)]
// mod tests {
//     // #[test]
//     // fn test_1() {
//     //     use super::*;
//     //     let tree = parse_tree_from_file("/home/marcospb19/dotao.tree");
//     //     println!("{:#?}", tree);
//     //     assert!(true);
//     // }

//     #[test]
//     fn test_logos() {
//         use super::*;
//         let tree = parse_tree_from_file_logos("/home/marcospb19/dotao.tree");
//         println!("{:#?}", tree);
//         assert!(true);
//     }
// }
