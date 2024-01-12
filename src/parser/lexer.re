/*!re2c
    re2c:encoding:utf8       = 1;
    re2c:encoding-policy     = substitute;

    re2c:define:YYCTYPE      = u8;
    re2c:define:YYPEEK       = "*s.get_unchecked(cursor)";
    re2c:define:YYSKIP       = "cursor += 1;";
    re2c:define:YYBACKUP     = "marker = cursor;";
    re2c:define:YYRESTORE    = "cursor = marker;";
    re2c:define:YYBACKUPCTX  = "ctxmarker = cursor;";
    re2c:define:YYRESTORECTX = "cursor = ctxmarker;";
    re2c:yyfill:enable       = 0;
    re2c:indent:string       = '    ';
    re2c:indent:top          = 1;
*/

use std::str;

use super::ast::Node;

pub(crate) struct LexOne {
    pub(crate) consume: usize,
    pub(crate) node: Option<Node>,
}

pub(crate) fn lex_one(s: &[u8]) -> LexOne {
    let mut cursor = 0;
    let mut marker = 0;
/*!re2c
    ";" [^\r\n]* { return LexOne { consume: cursor, node: None }; }
    [ \t\r\n]+ { return LexOne { consume: cursor, node: None }; }
    [a-zA-Z*_-][a-zA-Z0-9*_-]* { return LexOne { consume: cursor, node: Some(Node::Symbol(str::from_utf8(&s[..cursor]).unwrap().to_string())) }; }
    * { return LexOne { consume: 0, node: None }; }
*/
}
