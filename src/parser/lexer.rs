/* Generated by re2c 3.1 on Fri Jan 12 20:28:33 2024 */

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
}

fn token(kind: TokenKind, s: &[u8], n: usize) -> Token {
    Token {
        kind,
        excerpt: &s[..n],
    }
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

    {
        #[allow(unused_assignments)]
        let mut yych: u8 = 0;
        let mut yyaccept: usize = 0;
        let mut yystate: usize = 0;
        'yyl: loop {
            match yystate {
                0 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    cursor += 1;
                    match yych {
                        0x09..=0x0A | 0x0D | 0x20 => {
                            yystate = 2;
                            continue 'yyl;
                        }
                        0x22 => {
                            yystate = 4;
                            continue 'yyl;
                        }
                        0x27 => {
                            yystate = 6;
                            continue 'yyl;
                        }
                        0x28 => {
                            yystate = 7;
                            continue 'yyl;
                        }
                        0x29 => {
                            yystate = 8;
                            continue 'yyl;
                        }
                        0x2A | 0x2D | 0x41..=0x5A | 0x5F | 0x61..=0x7A => {
                            yystate = 9;
                            continue 'yyl;
                        }
                        0x30 => {
                            yystate = 11;
                            continue 'yyl;
                        }
                        0x31..=0x39 => {
                            yystate = 13;
                            continue 'yyl;
                        }
                        0x3B => {
                            yystate = 15;
                            continue 'yyl;
                        }
                        0x5B => {
                            yystate = 17;
                            continue 'yyl;
                        }
                        0x5D => {
                            yystate = 18;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 1;
                            continue 'yyl;
                        }
                    }
                }
                1 => {
                    return err(s);
                }
                2 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x09..=0x0A | 0x0D | 0x20 => {
                            cursor += 1;
                            yystate = 2;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 3;
                            continue 'yyl;
                        }
                    }
                }
                3 => {
                    return skip(s, cursor);
                }
                4 => {
                    yyaccept = 0;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x01..=0x21 | 0x23..=0x5B | 0x5D..=0x7F => {
                            cursor += 1;
                            yystate = 4;
                            continue 'yyl;
                        }
                        0x22 => {
                            cursor += 1;
                            yystate = 19;
                            continue 'yyl;
                        }
                        0x5C => {
                            cursor += 1;
                            yystate = 20;
                            continue 'yyl;
                        }
                        0xC2..=0xDF => {
                            cursor += 1;
                            yystate = 22;
                            continue 'yyl;
                        }
                        0xE0 => {
                            cursor += 1;
                            yystate = 23;
                            continue 'yyl;
                        }
                        0xE1..=0xEC | 0xEE..=0xEF => {
                            cursor += 1;
                            yystate = 24;
                            continue 'yyl;
                        }
                        0xED => {
                            cursor += 1;
                            yystate = 25;
                            continue 'yyl;
                        }
                        0xF0 => {
                            cursor += 1;
                            yystate = 26;
                            continue 'yyl;
                        }
                        0xF1..=0xF3 => {
                            cursor += 1;
                            yystate = 27;
                            continue 'yyl;
                        }
                        0xF4 => {
                            cursor += 1;
                            yystate = 28;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 5;
                            continue 'yyl;
                        }
                    }
                }
                5 => {
                    return token(TokenKind::String, s, cursor);
                }
                6 => {
                    return token(TokenKind::Quote, s, cursor);
                }
                7 => {
                    return token(TokenKind::ListStart, s, cursor);
                }
                8 => {
                    return token(TokenKind::ListEnd, s, cursor);
                }
                9 => {
                    yyaccept = 1;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x2A | 0x2D | 0x30..=0x39 | 0x41..=0x5A | 0x5F | 0x61..=0x7A => {
                            cursor += 1;
                            yystate = 9;
                            continue 'yyl;
                        }
                        0x2F => {
                            cursor += 1;
                            yystate = 29;
                            continue 'yyl;
                        }
                        0x3A => {
                            cursor += 1;
                            yystate = 30;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 10;
                            continue 'yyl;
                        }
                    }
                }
                10 => {
                    return token(TokenKind::Symbol, s, cursor);
                }
                11 => {
                    yyaccept = 2;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x78 => {
                            cursor += 1;
                            yystate = 32;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 14;
                            continue 'yyl;
                        }
                    }
                }
                12 => {
                    return token(TokenKind::Number, s, cursor);
                }
                13 => {
                    yyaccept = 2;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    yystate = 14;
                    continue 'yyl;
                }
                14 => match yych {
                    0x2E => {
                        cursor += 1;
                        yystate = 31;
                        continue 'yyl;
                    }
                    0x30..=0x39 | 0x5F => {
                        cursor += 1;
                        yystate = 13;
                        continue 'yyl;
                    }
                    _ => {
                        yystate = 12;
                        continue 'yyl;
                    }
                },
                15 => {
                    yyaccept = 3;
                    marker = cursor;
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x01..=0x09 | 0x0B..=0x0C | 0x0E..=0x7F => {
                            cursor += 1;
                            yystate = 15;
                            continue 'yyl;
                        }
                        0xC2..=0xDF => {
                            cursor += 1;
                            yystate = 33;
                            continue 'yyl;
                        }
                        0xE0 => {
                            cursor += 1;
                            yystate = 34;
                            continue 'yyl;
                        }
                        0xE1..=0xEC | 0xEE..=0xEF => {
                            cursor += 1;
                            yystate = 35;
                            continue 'yyl;
                        }
                        0xED => {
                            cursor += 1;
                            yystate = 36;
                            continue 'yyl;
                        }
                        0xF0 => {
                            cursor += 1;
                            yystate = 37;
                            continue 'yyl;
                        }
                        0xF1..=0xF3 => {
                            cursor += 1;
                            yystate = 38;
                            continue 'yyl;
                        }
                        0xF4 => {
                            cursor += 1;
                            yystate = 39;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 16;
                            continue 'yyl;
                        }
                    }
                }
                16 => {
                    return skip(s, cursor);
                }
                17 => {
                    return token(TokenKind::VecStart, s, cursor);
                }
                18 => {
                    return token(TokenKind::VecEnd, s, cursor);
                }
                19 => {
                    yystate = 5;
                    continue 'yyl;
                }
                20 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x22 | 0x5C | 0x6E | 0x72 | 0x74 => {
                            cursor += 1;
                            yystate = 4;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                21 => {
                    cursor = marker;
                    match yyaccept {
                        0 => {
                            yystate = 5;
                            continue 'yyl;
                        }
                        1 => {
                            yystate = 10;
                            continue 'yyl;
                        }
                        2 => {
                            yystate = 12;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 16;
                            continue 'yyl;
                        }
                    }
                }
                22 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0xBF => {
                            cursor += 1;
                            yystate = 4;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                23 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0xA0..=0xBF => {
                            cursor += 1;
                            yystate = 22;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                24 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0xBF => {
                            cursor += 1;
                            yystate = 22;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                25 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0x9F => {
                            cursor += 1;
                            yystate = 22;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                26 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x90..=0xBF => {
                            cursor += 1;
                            yystate = 24;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                27 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0xBF => {
                            cursor += 1;
                            yystate = 24;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                28 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0x8F => {
                            cursor += 1;
                            yystate = 24;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                29 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x2A | 0x2D | 0x41..=0x5A | 0x5F | 0x61..=0x7A => {
                            cursor += 1;
                            yystate = 40;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                30 => {
                    return token(TokenKind::SymbolColon, s, cursor);
                }
                31 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x30..=0x39 | 0x5F => {
                            cursor += 1;
                            yystate = 41;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                32 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x30..=0x39 | 0x41..=0x46 | 0x5F | 0x61..=0x66 => {
                            cursor += 1;
                            yystate = 42;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                33 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0xBF => {
                            cursor += 1;
                            yystate = 15;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                34 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0xA0..=0xBF => {
                            cursor += 1;
                            yystate = 33;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                35 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0xBF => {
                            cursor += 1;
                            yystate = 33;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                36 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0x9F => {
                            cursor += 1;
                            yystate = 33;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                37 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x90..=0xBF => {
                            cursor += 1;
                            yystate = 35;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                38 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0xBF => {
                            cursor += 1;
                            yystate = 35;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                39 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x80..=0x8F => {
                            cursor += 1;
                            yystate = 35;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 21;
                            continue 'yyl;
                        }
                    }
                }
                40 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x2A | 0x2D | 0x30..=0x39 | 0x41..=0x5A | 0x5F | 0x61..=0x7A => {
                            cursor += 1;
                            yystate = 40;
                            continue 'yyl;
                        }
                        0x3A => {
                            cursor += 1;
                            yystate = 30;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 10;
                            continue 'yyl;
                        }
                    }
                }
                41 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x30..=0x39 | 0x5F => {
                            cursor += 1;
                            yystate = 41;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 12;
                            continue 'yyl;
                        }
                    }
                }
                42 => {
                    yych = unsafe {
                        if cursor < len {
                            *s.get_unchecked(cursor)
                        } else {
                            0
                        }
                    };
                    match yych {
                        0x30..=0x39 | 0x41..=0x46 | 0x5F | 0x61..=0x66 => {
                            cursor += 1;
                            yystate = 42;
                            continue 'yyl;
                        }
                        _ => {
                            yystate = 43;
                            continue 'yyl;
                        }
                    }
                }
                43 => {
                    return token(TokenKind::Number, s, cursor);
                }
                _ => {
                    panic!("internal lexer error")
                }
            }
        }
    }
}
