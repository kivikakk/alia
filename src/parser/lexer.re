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

pub(crate) enum TokenKind {
    Whitespace,
    Atom,
    AtomColon,
    Number,
    String,
    ListStart,
    ListEnd,
    VecStart,
    VecEnd,
}

pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) excerpt: &'a [u8],
}

fn token(kind: TokenKind, s: &[u8], n: usize) -> Token {
    Token { kind, excerpt: &s[..n] }
}

fn skip(s: &[u8], n: usize) -> Token {
    token(TokenKind::Whitespace, s, n)
}

fn err(s: &[u8]) -> Token {
    skip(s, 0)
}

pub(crate) fn lex_one(s: &[u8]) -> Token {
    let mut cursor = 0;
    let mut marker = 0;
    let len = s.len();
/*!re2c

    ";" [^\r\n\x00]* { return skip(s, cursor); }

    [ \t\r\n]+ { return skip(s, cursor); }

    [a-zA-Z*_-][a-zA-Z0-9*_-]* ("/" [a-zA-Z*_-][a-zA-Z0-9*_-]*)? ":" { return token(TokenKind::AtomColon, s, cursor); }
    [a-zA-Z*_-][a-zA-Z0-9*_-]* ("/" [a-zA-Z*_-][a-zA-Z0-9*_-]*)? { return token(TokenKind::Atom, s, cursor); }

    "0x" [0-9a-fA-F_]+ { return token(TokenKind::Number, s, cursor); }
    [0-9][0-9_]* ("." [0-9_]+)? { return token(TokenKind::Number, s, cursor); }

    ["] ([^\\"\x00] | [\\][rnt\\"])* ["]? { return token(TokenKind::String, s, cursor); }

    "(" { return token(TokenKind::ListStart, s, cursor); }
    ")" { return token(TokenKind::ListEnd, s, cursor); }

    "[" { return token(TokenKind::VecStart, s, cursor); }
    "]" { return token(TokenKind::VecEnd, s, cursor); }

    * { return err(s); }

*/
}
