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

pub(crate) fn lex_one(s: &[u8]) -> Option<LexOne> {
    let mut cursor = 0;
/*!re2c
    [ \t\r\n]+ { return Some(LexOne { consume: cursor, node: None }); }
    [a-zA-Z*_-]+ { return Some(LexOne { consume: cursor, node: Some(Node::Symbol(str::from_utf8(&s[..cursor]).unwrap().to_string())) }); }
    * { return None; }
*/
}
