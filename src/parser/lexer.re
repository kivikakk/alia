/*!re2c
    re2c:encoding:utf8       = 1;
    re2c:encoding-policy     = substitute;

    re2c:define:YYCTYPE      = u8;
    re2c:define:YYPEEK       = "if cursor < len { *s.get_unchecked(cursor) } else { 0 }";
    re2c:define:YYSKIP       = "cursor += 1;";
    re2c:define:YYBACKUP     = "marker = cursor;";
    re2c:define:YYRESTORE    = "cursor = marker;";
    re2c:define:YYBACKUPCTX  = "ctxmarker = cursor;";
    re2c:define:YYRESTORECTX = "cursor = ctxmarker;";
    re2c:yyfill:enable       = 0;
    re2c:indent:string       = '    ';
    re2c:indent:top          = 1;
*/

use super::Loc;

pub(crate) enum TokenKind {
    Whitespace,
    Symbol,
    SymbolColon,
    Number,
    String,
    ListStart,
    ListEnd,
    VecStart,
    VecEnd,
    Quote,
}

pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) excerpt: &'a [u8],
    pub(crate) start: Loc,
    pub(crate) end: Loc,
}

fn token(kind: TokenKind, s: &[u8], n: usize, mut loc: Loc) -> Token {
    let start = loc;
    let excerpt = &s[..n];
    for c in excerpt {
        match c {
            b'\n' => loc = Loc(loc.0 + 1, 0),
            _ => loc.1 += 1,
        }
    }
    Token {
        kind,
        excerpt,
        start,
        end: loc,
    }
}

fn skip(s: &[u8], n: usize, loc: Loc) -> Token {
    token(TokenKind::Whitespace, s, n, loc)
}

fn err(s: &[u8]) -> Token {
    skip(s, 0, Loc(0, 0))
}

pub(super) fn lex_one(s: &[u8], loc: Loc) -> Token {
    let mut cursor = 0;
    let mut marker = 0;
    let len = s.len();
/*!re2c

    ";" [^\r\n\x00]* { return skip(s, cursor, loc); }

    [ \t\r\n]+ { return skip(s, cursor, loc); }

    symchar = [a-zA-Z*_<>!=+-];
    symchartail = symchar | [0-9.];

    "/:" { return token(TokenKind::SymbolColon, s, cursor, loc); }
    "/" { return token(TokenKind::Symbol, s, cursor, loc); }

    symchar symchartail* ("/" symchar symchartail*)? ":" { return token(TokenKind::SymbolColon, s, cursor, loc); }
    symchar symchartail* ("/" symchar symchartail*)? { return token(TokenKind::Symbol, s, cursor, loc); }

    "0x" [0-9a-fA-F_]+ { return token(TokenKind::Number, s, cursor, loc); }
    [0-9][0-9_]* ("." [0-9_]+)? { return token(TokenKind::Number, s, cursor, loc); }

    ["] ([^\\"\x00] | [\\][rnt\\"])* ["]? { return token(TokenKind::String, s, cursor, loc); }

    "(" { return token(TokenKind::ListStart, s, cursor, loc); }
    ")" { return token(TokenKind::ListEnd, s, cursor, loc); }

    "[" { return token(TokenKind::VecStart, s, cursor, loc); }
    "]" { return token(TokenKind::VecEnd, s, cursor, loc); }

    "'" { return token(TokenKind::Quote, s, cursor, loc); }

    * { return err(s); }

*/
}
