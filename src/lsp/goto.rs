use lsp_server::ResponseError;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};

use super::LspState;

pub(super) fn handle(
    _params: GotoDefinitionParams,
    _ls: &mut LspState,
) -> Result<Option<GotoDefinitionResponse>, ResponseError> {
    Ok(Some(GotoDefinitionResponse::Array(vec![])))
}
