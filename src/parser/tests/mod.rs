#![cfg(test)]

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use super::{Document, Node};

fn assert_roundtrips<T>(s: &str)
where
    T: FromStr + Display + PartialEq + Debug,
    T::Err: Debug,
{
    println!("should parse: {s}");
    let first = s.parse::<T>().expect(&format!("input should parse: {s}"));
    let rendered = format!("{}", first);
    let second = rendered
        .parse::<T>()
        .expect("rerendered input should parse");
    assert_eq!(first, second);
}

#[test]
fn simples_all_parse() {
    let mut expected: Option<bool> = None;

    for line in include_str!("simple.lia").lines() {
        if line == "" || line.starts_with(";") {
            continue;
        }
        if line == ":)" {
            expected = Some(true);
            continue;
        }
        if line == ":(" {
            expected = Some(false);
            continue;
        }

        if !expected.expect("should declare which way to assert") {
            println!("shouldn't parse: {line}");
            line.parse::<Node>().expect_err("input shouldn't parse");
        } else {
            assert_roundtrips::<Node>(line);
        }
    }
}

#[test]
fn document_parses() {
    assert_roundtrips::<Document>("a b");
    assert_roundtrips::<Document>(
        r#"
        (ns abc)
        (abc/def [ghi: jkl])
    "#,
    );
}
