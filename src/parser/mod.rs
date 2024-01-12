mod ast;
mod lexer;
mod tests;

use std::fmt::{Debug, Display};
use std::str;

pub(crate) use self::ast::Node;
use self::lexer::{lex_one, Token, TokenKind};

pub(crate) enum ParseNodeError {
    Empty,
    Unfinished,
    Unexpected(char),
    Multiple,
    Number,
    String,
    Other,
}

impl Display for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("input is empty"),
            Self::Unfinished => f.write_str("input appears unfinished"),
            Self::Unexpected(c) => write!(f, "unexpected {c:?}"),
            Self::Multiple => f.write_str("multiple forms found"),
            Self::Number => f.write_str("number parse fail"),
            Self::String => f.write_str("string parse fail"),
            Self::Other => f.write_str("other unsorted error"),
        }
    }
}

impl Debug for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

struct Parser {
    stack: Vec<PE>,
    result: Option<Node>,
}

enum PE {
    List(Vec<Node>),
    Vec(Vec<Node>),
    Quote,
}

impl Parser {
    fn new() -> Self {
        Parser {
            stack: vec![],
            result: None,
        }
    }

    fn atom(&mut self, mut node: Node) -> Result<(), ParseNodeError> {
        loop {
            match self.stack.last_mut() {
                None => {
                    if self.result.is_some() {
                        return Err(ParseNodeError::Multiple);
                    }
                    self.result = Some(node);
                    return Ok(());
                }
                Some(PE::List(ns)) | Some(PE::Vec(ns)) => {
                    ns.push(node);
                    return Ok(());
                }
                Some(PE::Quote) => {
                    self.stack.pop();
                    node = Node::List(vec![Node::Symbol("quote".to_string()), node]);
                    // loop
                }
            }
        }
    }

    fn list_start(&mut self) -> Result<(), ParseNodeError> {
        if self.result.is_some() {
            return Err(ParseNodeError::Multiple);
        }

        self.stack.push(PE::List(vec![]));
        Ok(())
    }

    fn list_end(&mut self) -> Result<(), ParseNodeError> {
        let ns = match self.stack.pop().ok_or(ParseNodeError::Other)? {
            PE::List(ns) => ns,
            _ => return Err(ParseNodeError::Other),
        };
        self.atom(Node::List(ns))
    }

    fn vec_start(&mut self) -> Result<(), ParseNodeError> {
        if self.result.is_some() {
            return Err(ParseNodeError::Multiple);
        }

        self.stack.push(PE::Vec(vec![]));
        Ok(())
    }

    fn vec_end(&mut self) -> Result<(), ParseNodeError> {
        let ns = match self.stack.pop().ok_or(ParseNodeError::Other)? {
            PE::Vec(ns) => ns,
            _ => return Err(ParseNodeError::Other),
        };
        self.atom(Node::Vec(ns))
    }

    fn quote(&mut self) -> Result<(), ParseNodeError> {
        if let Some(result) = self.result.take() {
            self.result = Some(Self::quoted_form(result));
        } else {
            self.stack.push(PE::Quote);
        }
        Ok(())
    }

    fn quoted_form(node: Node) -> Node {
        Node::List(vec![Node::Symbol("quote".to_string()), node])
    }

    fn finish(self) -> Result<Node, ParseNodeError> {
        if !self.stack.is_empty() {
            return Err(ParseNodeError::Unfinished);
        }
        self.result.ok_or(ParseNodeError::Empty)
    }
}

pub(crate) fn parse(s: &str) -> Result<Node, ParseNodeError> {
    let s = s.as_bytes();
    let mut offset = 0;
    let mut parser = Parser::new();

    while offset < s.len() {
        let Token { kind, excerpt } = lex_one(&s[offset..]);
        let consume = excerpt.len();
        if consume == 0 {
            return Err(ParseNodeError::Unexpected(s[offset] as char));
        }

        offset += consume;

        match kind {
            TokenKind::Whitespace => {}
            TokenKind::Symbol => parser.atom(Node::Symbol(parse_symbol(excerpt)?))?,
            TokenKind::SymbolColon => {
                // This is esoteric, but I'll stick with it for now.
                // Elixir does [a: b] => [{:a, b}].  Ruby does f(a: b) => f({ a: b }) (-ish,
                // modulo all the Ruby 3 kwargs improvements).
                // Here we do [a: b] => ['a b].
                parser.atom(Node::Symbol(parse_symbol(&excerpt[..excerpt.len() - 1])?))?
            }
            TokenKind::Number => parser.atom(Node::Number(parse_number(excerpt)?))?,
            TokenKind::String => parser.atom(Node::String(parse_string(excerpt)?))?,
            TokenKind::ListStart => parser.list_start()?,
            TokenKind::ListEnd => parser.list_end()?,
            TokenKind::VecStart => parser.vec_start()?,
            TokenKind::VecEnd => parser.vec_end()?,
            TokenKind::Quote => parser.quote()?,
        }
    }

    parser.finish()
}

fn parse_symbol(a: &[u8]) -> Result<String, ParseNodeError> {
    Ok(unsafe { str::from_utf8_unchecked(a) }.to_string())
}

fn parse_number(s: &[u8]) -> Result<u64, ParseNodeError> {
    unsafe { str::from_utf8_unchecked(s) }
        .parse()
        .map_err(|_| ParseNodeError::Number)
}

fn parse_string(s: &[u8]) -> Result<String, ParseNodeError> {
    let len = s.len();
    let mut r = String::new();
    let mut i = 0;

    while i < len {
        match s[i] {
            b'\\' => {
                i += 1;
                if i == len {
                    return Err(ParseNodeError::String);
                }
                match s[i] {
                    b'\\' | b'"' => r.push(s[i] as char),
                    b't' => r.push('\t'),
                    b'r' => r.push('\r'),
                    b'n' => r.push('\n'),
                    _ => return Err(ParseNodeError::String),
                }
            }
            b'"' => {
                i += 1;
                if i == 1 {
                    continue;
                }
                if i == len {
                    return Ok(r);
                }
                // should really never happen
                return Err(ParseNodeError::String);
            }
            b => r.push(b as char),
        }

        i += 1;
    }

    Err(ParseNodeError::Unfinished)
}
