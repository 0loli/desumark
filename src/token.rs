#[derive(Debug, PartialEq)]
pub enum Token {
    Char(char),
    Start,
    End,
    Space,
    Backslash,
    Quote,
    SingleQuote,
    Equal,
    Ignore,
}

impl Into<char> for Token {
    fn into(self) -> char {
        use Token::*;

        match self {
            Char(c) => c,
            Start => '[',
            End => ']',
            Space => ' ',
            Backslash => '/',
            Quote => '"',
            SingleQuote => '\'',
            Equal => '=',
            Ignore => '\\',
        }
    }
}
