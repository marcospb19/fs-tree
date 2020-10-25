// State machine parser from tokens
//
// # States:
// We don't need state recording for parsing symlinks because of it is a trivial
// sequential 3 element combination.
//
// ParserScopeState suggests if we are inside of an scope, commas are only
// supported inside of it
//
// After groups we expect line break!
//
// We are busy to read until we find a LexToken::Separator or the end of the
// sequence
//
// Insane state machine, omfg

use crate::{lexer::SpannedLexToken, GroupsMap, LexToken};
use file_structure::File;
use std::collections::VecDeque;
type Stack<T> = VecDeque<T>;

const DEFAULT_GROUP: &str = "main";

enum ParserState {
    Clear,
    Busy,
}

pub fn parse_tokens(spanned_tokens: Vec<SpannedLexToken>) -> GroupsMap {
    let map = GroupsMap::new();
    let i = 0;

    let mut file_stack: Stack<File> = Stack::new();
    let mut read_state = ParserState::Clear;
    let mut current_group: &str = DEFAULT_GROUP;

    while i < spanned_tokens.len() {
        let token = &spanned_tokens[i].0;
        let range = &spanned_tokens[i].1;

        let depth = file_stack.len();
        match token {
            LexToken::Value(value) => {
                if let ParserState::Busy = read_state {
                    panic!("{:?}", range);
                }
                read_state = ParserState::Busy;

                //...
            },

            LexToken::Separator(separator) => {
                if depth == 0 && *separator == ',' {
                    panic!();
                }
                read_state = ParserState::Clear;
            },

            LexToken::Group(group) => {
                if map.get(current_group).is_some() {
                    // map[current_group].append()
                }
            },

            _ => {},
        }
    }

    map
}
