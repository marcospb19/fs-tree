// State machine parser from tokens
//
// # States:
// We don't need state recording for parsing symlinks because of it is a trivial
// sequential 3 element combination.
//
// ParserScopeState suggests if we are inside of an scope, commas are only
// supported inside of it
//
// After groups we expect line break!!!!
use crate::{lexer::SpannedLexToken, GroupsMap, LexToken};
use file_structure::{File, FileType};
use std::{fmt, path::PathBuf, result};

type Stack<T> = Vec<T>;

const DEFAULT_GROUP: &str = "main";

enum ParserState {
    Clear,
    Busy,
}

#[derive(Debug)]
pub struct ParserError {
    line: usize,
    column: usize,
    kind: ParserErrorKind,
}

impl ParserError {
    pub(crate) fn new(line: usize, column: usize, kind: ParserErrorKind) -> Self {
        ParserError { line, column, kind }
    }
}

#[derive(Debug)]
pub enum ParserErrorKind {
    BracketUnclosed,
    BracketUnexpectedClose,
    BracketUnexpectedOpen,
    CommasOutsideOfBrackets,
    MissingSymlinkTarget,
}

pub type ParserResult<T> = result::Result<T, ParserError>;

fn update_map_group(map: &mut GroupsMap, group: String, files: &mut Stack<File>) {
    // If group is already there, append
    if let Some(group) = map.get_mut(&group) {
        group.append(files);
    } else {
        // Else, move vec
        map.insert(group, files.to_vec());
        *files = Stack::new();
    }
}

pub fn parse_tokens(spanned_tokens: Vec<SpannedLexToken>) -> ParserResult<GroupsMap> {
    let mut map = GroupsMap::new();

    let mut current_line = 1;
    let mut current_line_start_index = 0;

    let mut file_stack: Stack<File> = Stack::new();
    let mut quantity_stack: Stack<usize> = vec![0];
    let mut read_state = ParserState::Clear;
    let mut current_group = String::from(DEFAULT_GROUP);
    let mut already_read_some_lmao = false;
    let mut brackets_open_position = vec![];

    let mut tokens_iter = spanned_tokens.into_iter().peekable();
    let mut depth = 0;

    while let Some((token, range)) = tokens_iter.next() {
        let current_column = range.start - current_line_start_index;
        match &token {
            LexToken::Value(value) => {
                *quantity_stack.last_mut().unwrap() += 1;

                if let ParserState::Busy = read_state {
                    panic!("busy when read this token: '{:?}'", token);
                    // panic!("{:?}", range);
                }
                read_state = ParserState::Busy;
                file_stack.push(File::new(value, FileType::Regular));
                already_read_some_lmao = true;

                if let Some((LexToken::SymlinkArrow, _r1)) = tokens_iter.peek() {
                    if let Some((LexToken::Value(target), _r2)) = tokens_iter.nth(1) {
                        if file_stack.last().unwrap().file_type.is_dir() {
                            unimplemented!("straight up panic");
                        }

                        let path = file_stack.pop().unwrap().path;
                        file_stack.push(File::new(path, FileType::Symlink(PathBuf::from(target))));
                    } else {
                        panic!("Was expecting the target of the symlink");
                        // return Err(ParserError::new(
                        //     0,
                        //     0,
                        //     ParserErrorKind::MissingSymlinkTarget,
                        // ))
                    }
                }
            },

            LexToken::DoubleDots => {
                // optional
                println!("Double dots!");
            },

            LexToken::OpenBracket => {
                brackets_open_position.push((current_line, current_column));
                read_state = ParserState::Clear;
                // If trying to open nothing fail
                if !already_read_some_lmao {
                    return Err(ParserError::new(
                        current_line,
                        current_column,
                        ParserErrorKind::BracketUnexpectedOpen,
                    ));
                }

                assert!(!file_stack.is_empty());

                depth += 1;
                file_stack.last_mut().unwrap().file_type = FileType::Directory(vec![]);
                quantity_stack.push(0);
                already_read_some_lmao = false;
            },

            LexToken::CloseBracket => {
                brackets_open_position.pop();

                if depth == 0 {
                    return Err(ParserError::new(
                        current_line,
                        current_column,
                        ParserErrorKind::BracketUnexpectedClose,
                    ));
                }

                already_read_some_lmao = true;
                depth -= 1;
                let mut vec: Vec<File> = vec![];

                for _ in 0..quantity_stack.pop().unwrap() {
                    vec.push(file_stack.pop().unwrap());
                }

                let current_last = file_stack.pop().unwrap();
                file_stack.push(File::new(current_last.path, FileType::Directory(vec)));
            },

            LexToken::Separator(separator) => {
                if depth == 0 && *separator == ',' {
                    return Err(ParserError::new(
                        current_line,
                        current_column,
                        ParserErrorKind::CommasOutsideOfBrackets,
                    ));
                }
                read_state = ParserState::Clear;

                if *separator == '\n' {
                    current_line += 1;
                    current_line_start_index = range.start;
                }
            },

            LexToken::Group(group) => {
                // Add everything from last group
                update_map_group(&mut map, current_group, &mut file_stack);
                current_group = group.into();
            },

            _ => {
                eprintln!("_ => {{ eprintln!(); }},");
            },
        }
    }

    if depth != 0 {
        // Only show the inner bracket problem for now, even if there are multiple
        // unclosed
        return Err(ParserError::new(
            brackets_open_position.last().unwrap().0, // wat
            brackets_open_position.last().unwrap().1, // wat
            ParserErrorKind::BracketUnclosed,
        ));
    }

    update_map_group(&mut map, current_group, &mut file_stack);
    Ok(map)
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "moao tree: ")?;
        if let ParserErrorKind::BracketUnclosed = self.kind {
            // "close those brackets man!!!",
            write!(f, "bracket at {}:{} is unclosed!", self.line, self.column)?;
        } else {
            write!(f, "moao tree: at {}:{}: ", self.line, self.column)?;
        }

        match self.kind {
            ParserErrorKind::BracketUnclosed => Ok(()),
            ParserErrorKind::BracketUnexpectedClose => {
                write!(f, "unexpected close brackets, what are you closing?")
            },
            ParserErrorKind::BracketUnexpectedOpen => {
                write!(f, "what are you trying to open there?????")
            },
            ParserErrorKind::CommasOutsideOfBrackets => {
                write!(f, "no commas alowed outsite of scopes")
            },
            ParserErrorKind::MissingSymlinkTarget => {
                write!(f, "arrow without the plim plimplimplim")
            },
        }
    }
}
