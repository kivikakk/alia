#![cfg(test)]

use super::Document;

fn assert_roundtrips(s: &str) {
    println!("should parse: {s}");
    let first = s
        .parse::<Document>()
        .unwrap_or_else(|_| panic!("input should parse: {s}"));
    let rendered = format!("{}", first);
    let second = rendered
        .parse::<Document>()
        .expect("rerendered input should parse");
    assert_eq!(first, second);
}

#[test]
fn simples_all_parse() {
    let mut expected: Option<bool> = None;

    for line in include_str!("simple.lia").lines() {
        if line.is_empty() || line.starts_with(';') {
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
            match line.parse::<Document>() {
                Ok(doc) => assert_ne!(1, doc.toplevels.len()),
                Err(_) => {}
            }
        } else {
            assert_roundtrips(line);
        }
    }
}

#[test]
fn document_parses() {
    assert_roundtrips("a b");
    assert_roundtrips(
        r#"
        (ns abc)
        (abc/def [ghi: jkl])
    "#,
    );
}
