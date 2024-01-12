#![cfg(test)]

use super::Node;

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
            println!("should parse: {line}");
            let first = line
                .parse::<Node>()
                .expect(&format!("input should parse: {line}"));
            let rendered = format!("{}", first);
            let second = rendered
                .parse::<Node>()
                .expect("rerendered input should parse");
            assert_eq!(first, second);
        }
    }
}
