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
    let nodes = doc.nodes_at((line as usize, character as usize));

    let mut value = String::new();
    writeln!(value, "# {}", "thing").unwrap();
    writeln!(value, "```lisp").unwrap();
    let mut range = None;
    for node in nodes {
        writeln!(value, "{node}").unwrap();
        range = Some(node.range.into());
    }
    writeln!(value, "```").unwrap();

    Ok(Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }),
        range,
    }))
}
