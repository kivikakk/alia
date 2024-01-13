use std::fmt::Write;

use lsp_server::ResponseError;
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Position};

use crate::parser::Document;

use super::LspState;

pub(super) fn handle(
    params: HoverParams,
    ls: &mut LspState,
) -> Result<Option<Hover>, ResponseError> {
    let uri = &params.text_document_position_params.text_document.uri;
    let content = ls.documents.get_document_content(uri, None).unwrap();
    let doc = match content.parse::<Document>() {
        Ok(doc) => doc,
        _ => return Ok(None),
    };

    let Position { line, character } = params.text_document_position_params.position;
    let mut nodes = doc
        .nodes_at((line as usize, character as usize))
        .into_iter();

    let closest = match nodes.next() {
        Some(n) => n,
        None => return Ok(None),
    };

    let mut value = String::new();
    writeln!(value, "# {closest}").unwrap();
    writeln!(
        value,
        "inferred type: **i don't know i'm just a baby squirrel**"
    )
    .unwrap();

    let mut first = true;
    while let Some(node) = nodes.next() {
        if first {
            writeln!(value, "## other forms under point").unwrap();
            first = false;
        }
        writeln!(value, "```lisp").unwrap();
        writeln!(value, "{node}").unwrap();
        writeln!(value, "```").unwrap();
    }

    Ok(Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }),
        range: Some(closest.range.into()),
    }))
}
