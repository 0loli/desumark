use crate::token::Token;
use std::iter::Peekable;

pub struct Tokenizer<I: Iterator<Item = char>> {
    pub ignore: bool,
    pub iterator: Peekable<I>,
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    pub fn new(i: I) -> Tokenizer<I> {
        Tokenizer {
            iterator: i.peekable(),
            ignore: false,
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        use Token::*;

        match self.iterator.peek() {
            None => None,
            Some('\\') => {
                if self.ignore {
                    Some(Char('\\'))
                } else {
                    self.ignore = true;
                    Some(Ignore)
                }
            }
            Some(c) => Some(if self.ignore {
                Char(*c)
            } else {
                match c {
                    '[' => Start,
                    ']' => End,
                    ' ' => Space,
                    '/' => Backslash,
                    '"' => Quote,
                    '\'' => SingleQuote,
                    '=' => Equal,
                    _ => Char(*c),
                }
            }),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        match self.iterator.next() {
            None => None,
            Some('\\') => {
                if self.ignore {
                    Some(Char('\\'))
                } else {
                    self.ignore = true;
                    Some(Ignore)
                }
            }
            Some(c) => Some(if self.ignore {
                self.ignore = false;
                Char(c)
            } else {
                match c {
                    '[' => Start,
                    ']' => End,
                    ' ' => Space,
                    '/' => Backslash,
                    '"' => Quote,
                    '\'' => SingleQuote,
                    '=' => Equal,
                    _ => Char(c),
                }
            }),
        }
    }
}
