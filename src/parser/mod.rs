mod document;
mod lexer;
mod loc;
mod node;
mod tests;

use std::fmt::{Debug, Display};
use std::str;

pub(crate) use self::document::Document;
pub(crate) use self::node::Node;

use self::lexer::{lex_one, Token, TokenKind};
use self::loc::{Loc, Range};
use self::node::NodeValue;

pub(crate) struct ParseError {
    pub(crate) kind: ParseErrorKind,
    pub(crate) range: Range,
}

impl ParseError {
    fn new<R: Into<Range>>(kind: ParseErrorKind, range: R) -> Self {
        ParseError {
            kind,
            range: range.into(),
        }
    }
}

pub(crate) enum ParseErrorKind {
    Empty,
    Unfinished,
    Unexpected(char),
    Multiple,
    Number,
    String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at [{}]", self.kind, self.range)
    }
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("input is empty"),
            Self::Unfinished => f.write_str("input appears unfinished"),
            Self::Unexpected(c) => write!(f, "unexpected {c:?}"),
            Self::Multiple => f.write_str("multiple forms found"),
            Self::Number => f.write_str("number parse fail"),
            Self::String => f.write_str("string parse fail"),
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

struct Parser {
    stack: Vec<PE>,
    result: Option<Node>,
}

enum PE {
    List(Vec<Node>, Range),
    Vec(Vec<Node>, Range),
    Quote(Range),
}

impl Parser {
    fn new() -> Self {
        Parser {
            stack: vec![],
            result: None,
        }
    }

    fn atom(&mut self, mut node: Node) -> Result<(), ParseError> {
        let origin = node.range;
        loop {
            match self.stack.last_mut() {
                None => {
                    if self.result.is_some() {
                        return Err(ParseError::new(ParseErrorKind::Multiple, origin));
                    }
                    self.result = Some(node);
                    return Ok(());
                }
                Some(PE::List(ns, _range)) | Some(PE::Vec(ns, _range)) => {
                    ns.push(node);
                    return Ok(());
                }
                Some(PE::Quote(range)) => {
                    let range_all = (range.0, node.range.1);
                    node = Node::new(
                        NodeValue::List(vec![
                            Node::new(NodeValue::Symbol("quote".to_string()), *range),
                            node,
                        ]),
                        range_all,
                    );
                    // fall through ...
                }
            }
            self.stack.pop();
            // ... and loop
        }
    }

    fn list_start<R: Into<Range>>(&mut self, range: R) -> Result<(), ParseError> {
        if self.result.is_some() {
            return Err(ParseError::new(ParseErrorKind::Multiple, range));
        }

        self.stack.push(PE::List(vec![], range.into()));
        Ok(())
    }

    fn list_end<R: Into<Range>>(&mut self, range: R) -> Result<(), ParseError> {
        let (ns, srange) = match self.stack.pop() {
            Some(PE::List(ns, srange)) => (ns, srange),
            _ => return Err(parse_error(ParseErrorKind::Unexpected(')'), range)),
        };
        self.atom(Node::new(NodeValue::List(ns), (srange.0, range.into().1)))
    }

    fn vec_start<R: Into<Range>>(&mut self, range: R) -> Result<(), ParseError> {
        if self.result.is_some() {
            return Err(ParseError::new(ParseErrorKind::Multiple, range));
        }

        self.stack.push(PE::Vec(vec![], range.into()));
        Ok(())
    }

    fn vec_end<R: Into<Range>>(&mut self, range: R) -> Result<(), ParseError> {
        let (ns, srange) = match self.stack.pop() {
            Some(PE::Vec(ns, srange)) => (ns, srange),
            _ => return Err(parse_error(ParseErrorKind::Unexpected(']'), range)),
        };
        self.atom(Node::new(NodeValue::Vec(ns), (srange.0, range.into().1)))
    }

    fn quote<R: Into<Range>>(&mut self, range: R) -> Result<(), ParseError> {
        if let Some(result) = self.result.take() {
            self.result = Some(Self::quoted_form(result, range));
        } else {
            self.stack.push(PE::Quote(range.into()));
        }
        Ok(())
    }

    fn quoted_form<R: Into<Range>>(node: Node, range: R) -> Node {
        let range = range.into();
        let range_all = (range.0, node.range.1);
        Node::new(
            NodeValue::List(vec![
                Node::new(NodeValue::Symbol("quote".to_string()), range),
                node,
            ]),
            range_all,
        )
    }

    fn try_finish(&mut self) -> Option<Node> {
        if !self.stack.is_empty() {
            None
        } else {
            self.result.take()
        }
    }

    fn eof<L: Into<Loc>>(self, loc: L) -> Result<Node, ParseError> {
        let loc = loc.into();
        if !self.stack.is_empty() {
            return Err(parse_error(ParseErrorKind::Unfinished, (loc, loc)));
        }
        self.result
            .ok_or_else(|| parse_error(ParseErrorKind::Empty, ((0, 0).into(), loc)))
    }
}

fn parse(s: &str, mut offset: usize, mut loc: Loc) -> Result<(Node, usize, Loc), ParseError> {
    let s = s.as_bytes();
    let mut parser = Parser::new();

    while offset < s.len() {
        let Token {
            kind,
            excerpt,
            start,
            end,
        } = lex_one(&s[offset..], loc);
        loc = end;
        let consume = excerpt.len();
        if consume == 0 {
            return Err(ParseError::new(
                ParseErrorKind::Unexpected(s[offset] as char),
                (start, end),
            ));
        }

        offset += consume;

        match kind {
            TokenKind::Whitespace => {}
            TokenKind::Symbol => parser.atom(Node::new(
                NodeValue::Symbol(parse_symbol(excerpt, (start, end))?),
                (start, end),
            ))?,
            TokenKind::SymbolColon => {
                // This is esoteric, but I'll stick with it for now.
                // Elixir does [a: b] => [{:a, b}].  Ruby does f(a: b) => f({ a: b }) (-ish,
                // modulo all the Ruby 3 kwargs improvements).
                // Here we do [a: b] => ['a b].
                parser.atom(Node::new(
                    NodeValue::Symbol(parse_symbol(&excerpt[..excerpt.len() - 1], (start, end))?),
                    (start, end),
                ))?
            }
            TokenKind::Number => parser.atom(Node::new(
                NodeValue::Number(parse_number(excerpt, (start, end))?),
                (start, end),
            ))?,
            TokenKind::String => parser.atom(Node::new(
                NodeValue::String(parse_string(excerpt, (start, end))?),
                (start, end),
            ))?,
            TokenKind::ListStart => parser.list_start((start, end))?,
            TokenKind::ListEnd => parser.list_end((start, end))?,
            TokenKind::VecStart => parser.vec_start((start, end))?,
            TokenKind::VecEnd => parser.vec_end((start, end))?,
            TokenKind::Quote => parser.quote((start, end))?,
        }

        if let Some(result) = parser.try_finish() {
            return Ok((result, offset, loc));
        }
    }

    let result = parser.eof(loc)?;
    Ok((result, offset, loc))
}

fn parse_symbol<R: Into<Range>>(a: &[u8], _range: R) -> Result<String, ParseError> {
    Ok(unsafe { str::from_utf8_unchecked(a) }.to_string())
}

fn parse_number<R: Into<Range>>(s: &[u8], range: R) -> Result<u64, ParseError> {
    unsafe { str::from_utf8_unchecked(s) }
        .parse()
        .map_err(|_| parse_error(ParseErrorKind::Number, range))
}

fn parse_string<R: Into<Range>>(s: &[u8], range: R) -> Result<String, ParseError> {
    let len = s.len();
    let mut r = String::new();
    let mut i = 0;

    while i < len {
        match s[i] {
            b'\\' => {
                i += 1;
                if i == len {
                    return Err(parse_error(ParseErrorKind::String, range));
                    // XXX
                }
                match s[i] {
                    b'\\' | b'"' => r.push(s[i] as char),
                    b't' => r.push('\t'),
                    b'r' => r.push('\r'),
                    b'n' => r.push('\n'),
                    _ => return Err(parse_error(ParseErrorKind::String, range)),
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
                return Err(parse_error(ParseErrorKind::String, range));
            }
            b => r.push(b as char),
        }

        i += 1;
    }

    Err(parse_error(ParseErrorKind::Unfinished, range))
}

fn parse_error<R: Into<Range>>(kind: ParseErrorKind, range: R) -> ParseError {
    ParseError {
        kind,
        range: range.into(),
    }
}
