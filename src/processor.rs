use crate::{token::Token, tokenizer::Tokenizer, Error, Layout};
use std::collections::HashMap;

/// Main unit, that processes the text
pub struct Processor<'layout, E: 'static> {
    pub layout: &'layout Layout<E>,
    pub limited: bool,
}

impl<'tokenizer, 'layout, E: std::fmt::Display> Processor<'layout, E> {
    pub fn new(layout: &'layout Layout<E>, limited: bool) -> Self {
        Self { layout, limited }
    }

    pub fn process<I: Iterator<Item = char>>(
        &mut self,
        tokenizer: &mut Tokenizer<I>,
    ) -> Result<String, Error<E>> {
        use Token::*;
        let mut result = String::new();
        let mut counter = 0;

        while let Some(_w) = tokenizer.peek() {
            let mut buffer = String::new();
            if !self.limited {
                // we are starting from clear
                // we are writing raw text to result and waiting to tag start '['
                let tag_name_opened = loop {
                    if let Some(token) = tokenizer.next() {
                        counter += 1;
                        match token {
                            Start => {
                                if let Some(Backslash) = tokenizer.peek() {
                                    result.push('[');
                                } else {
                                    break true;
                                }
                            }

                            Ignore => {}

                            _ => result.push(token.into()),
                        }
                    } else {
                        break false;
                    }
                };
                // if no tags found
                if !tag_name_opened {
                    return Ok(result);
                }
            }

            // okay tag is found, lets read tag name and its arguments
            enum TagHeaderParsingState {
                TagName,
                ArgName,
                ArgValue,
            }

            // TAG HEADER
            let mut tag_name = String::new();
            let mut arguments = HashMap::new();
            let mut arg_name = String::new();
            let mut arg_value = String::new();
            let mut tag_header_state = TagHeaderParsingState::TagName;
            let mut quote = None;
            while let Some(token) = tokenizer.next() {
                counter += 1;

                if let Some(ref q) = quote {
                    if *q == token {
                        quote = None;
                    } else {
                        match tag_header_state {
                            TagHeaderParsingState::TagName => {
                                tag_name.push(token.into());
                            }

                            TagHeaderParsingState::ArgName => {
                                arg_name.push(token.into());
                            }

                            TagHeaderParsingState::ArgValue => {
                                arg_value.push(token.into());
                            }
                        }
                    }
                } else {
                    match token {
                        Char(c) => match tag_header_state {
                            TagHeaderParsingState::TagName => {
                                tag_name.push(c);
                            }

                            TagHeaderParsingState::ArgName => {
                                arg_name.push(c);
                            }

                            TagHeaderParsingState::ArgValue => {
                                arg_value.push(c);
                            }
                        },

                        End => {
                            if tag_name.is_empty() {
                                return Err(Error::TagNameEmpty);
                            }

                            match tag_header_state {
                                TagHeaderParsingState::TagName => {}

                                TagHeaderParsingState::ArgName => {
                                    arguments.insert(arg_name.clone(), None);
                                }

                                TagHeaderParsingState::ArgValue => {
                                    arguments.insert(arg_name.clone(), Some(arg_value.clone()));
                                }
                            }

                            break;
                        }

                        Space => match tag_header_state {
                            TagHeaderParsingState::TagName => {
                                tag_header_state = TagHeaderParsingState::ArgName
                            }

                            TagHeaderParsingState::ArgName => {
                                arguments.insert(arg_name.clone(), None);
                                arg_name.clear();
                            }

                            TagHeaderParsingState::ArgValue => {
                                tag_header_state = TagHeaderParsingState::ArgName;

                                arguments.insert(arg_name.clone(), Some(arg_value.clone()));
                                arg_name.clear();
                                arg_value.clear();
                            }
                        },

                        Equal => tag_header_state = TagHeaderParsingState::ArgValue,

                        Quote | SingleQuote => quote = Some(token),

                        Ignore => {}

                        _ => return Err(Error::TagHeaderInvalid),
                    }
                }
            }

            // INNER
            let _closed = false;
            'i: while let Some(token) = tokenizer.next() {
                counter += 1;
                match token {
                    Ignore => {}
                    Start => match tokenizer.peek() {
                        Some(v) => match v {
                            Ignore => {}
                            Backslash => {
                                let _ = tokenizer.next();
                                counter += 1;

                                let mut ending = String::new();

                                while let Some(t) = tokenizer.next() {
                                    counter += 1;
                                    match t {
                                        Char(ch) => ending.push(ch),

                                        End => {
                                            if !ending.is_empty() && ending != tag_name {
                                                return Err(Error::TagEndIncorrect);
                                            }

                                            break 'i;
                                        }

                                        Ignore => {}

                                        _ => return Err(Error::TagEndInvalid),
                                    }
                                }
                            }

                            Char(_c) => {
                                // parse inner
                                buffer.push_str(
                                    &Processor::new(self.layout, true).process(tokenizer)?,
                                );
                            }

                            _ => buffer.push(v.into()),
                        },
                        None => return Err(Error::TagNotEnded),
                    },

                    _ => buffer.push(token.into()),
                }
            }

            result.push_str(&self.layout.construct(tag_name, arguments, buffer)?);
            if self.limited {
                return Ok(result);
            }
        }
        // remain.a();

        Ok(result)
    }
}
