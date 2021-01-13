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
use crate::{
    flags::{Flag, FlagType, Flags},
    lexer::SpannedLexToken,
    File, FileType, GroupsMap, LexToken,
};

use std::{collections::HashMap, fmt, path::PathBuf, result};

type Stack<T> = Vec<T>;

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

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserErrorKind {
    BracketUnclosed,
    BracketUnexpectedClose,
    BracketUnexpectedOpen,
    CommasOutsideOfBrackets,
    MissingSymlinkTarget,
    GroupAfterGroup,
    FlagAfterFlag,
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

pub fn parse_tokens(
    spanned_tokens: Vec<SpannedLexToken>,
    original_text: &str,
) -> ParserResult<(GroupsMap, Vec<String>)> {
    let mut map = GroupsMap::new();

    let mut current_line = 1;
    let mut current_line_start_index = 0;

    let mut file_stack: Stack<File> = Stack::new();
    let mut quantity_stack: Stack<usize> = vec![0];
    let mut read_state = ParserState::Clear;
    let mut current_group = String::from("main");
    let mut already_read_some_lmao = false;
    let mut brackets_open_position = vec![];

    let mut tokens_iter = spanned_tokens.into_iter().peekable();
    let mut depth = 0;

    let mut group_flags = Vec::<Flag>::new();
    let mut last_flags = Vec::<Flag>::new();

    let mut group_order = vec!["main".to_string()];
    let mut groups_seen = HashMap::<String, ()>::new();
    groups_seen.insert("main".to_string(), ());

    // let mut pending_flags: Vec<String> = vec![];

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
                already_read_some_lmao = true;

                let mut file = File::new(value, FileType::Regular);
                let flags = last_flags
                    .into_iter()
                    .chain(group_flags.into_iter())
                    .map(|x| x.clone())
                    .collect();
                file.extra = Some(Flags::from(flags));

                // reinit
                last_flags = vec![];
                group_flags = vec![];

                if let Some((LexToken::SymlinkArrow, _r1)) = tokens_iter.peek() {
                    if let Some((LexToken::Value(target), _r2)) = tokens_iter.nth(1) {
                        file.file_type = FileType::<Flags>::Symlink(PathBuf::from(target));
                    } else {
                        panic!("Was expecting the target of the symlink");
                        // return Err(ParserError::new(0, 0,
                        // ParserErrorKind::MissingSymlinkTarget))
                    }
                }
                file_stack.push(file);
            },

            LexToken::DoubleDots => {
                // optional
            },

            LexToken::OpenBracket => {
                brackets_open_position.push((current_line, current_column));
                // Removed, need to study this again
                // assert!(matches!(read_state, ParserState::Busy), "WIP parser");
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
                file_stack.last_mut().unwrap().file_type = FileType::<Flags>::Directory(vec![]);
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
                file_stack.last_mut().expect("should").file_type =
                    FileType::<Flags>::Directory(vec);
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
                // Craziest shi ever... yeah
                groups_seen.entry(group.to_string()).or_insert_with(|| {
                    group_order.push(group.clone());
                    ()
                });

                // Add everything from PREVIOUS group
                update_map_group(&mut map, current_group, &mut file_stack);
                // Update the group for the next entries
                current_group = group.into();

                group_flags = last_flags
                    .into_iter()
                    .map(|x| Flag::new(x.name, FlagType::GroupInherited))
                    .collect();
                last_flags = Vec::default(); // reinit
            },

            // doing this
            LexToken::Flags(flags) => {
                if !last_flags.is_empty() {
                    return Err(ParserError::new(
                        current_line,
                        current_column,
                        ParserErrorKind::FlagAfterFlag,
                    ));
                }
                last_flags = flags
                    .iter()
                    .map(|x| Flag::new(x, FlagType::Direct))
                    .collect();

                println!("flags achadas: {:?}", last_flags);
            },

            // João Marcos!! Logos!! editar isso pls
            LexToken::SymlinkArrow => {
                panic!("Unexpected SymlinkArrow!");
            },

            LexToken::LexError => {
                eprintln!("LexError => '{}'", &original_text[range]);
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

    for value in map.iter_mut().flat_map(|(_key, value)| value.iter_mut()) {
        value.apply_to_children(|parent, child| {
            if let Some(parent_extra) = &parent.extra {
                let mut vec: Vec<Flag> = parent_extra
                    .clone()
                    .inner
                    .into_iter()
                    .map(|x| {
                        if let FlagType::Direct = x.flag_type {
                            Flag::new(x.name, FlagType::ParentInherited)
                        } else {
                            x
                        }
                    })
                    .collect();
                if let Some(asd) = &mut child.extra {
                    asd.inner.append(&mut vec);
                } else {
                    child.extra = Some(Flags::from(vec));
                }
            }
        });
    }
    Ok((map, group_order))
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
            ParserErrorKind::GroupAfterGroup => {
                write!(f, "Group after group problemo")
            },
            ParserErrorKind::FlagAfterFlag => {
                write!(f, "Flag after flag problemo")
            },
        }
    }
}
