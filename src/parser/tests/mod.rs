#![cfg(test)]

use super::Node;

#[test]
fn simples_all_parse() {
    for line in include_str!("simple.lia").lines() {
        if line == "" || line.starts_with(";") {
            continue;
        }
        let first = line.parse::<Node>().expect("original input should parse");
        let rendered = format!("{}", first);
        let second = rendered
            .parse::<Node>()
            .expect("rerendered input should parse");
        assert_eq!(first, second);
    }
}
