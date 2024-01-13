use lsp_server::ResponseError;
use lsp_types::{
    CodeActionParams, CodeActionProviderCapability, CodeActionResponse, Command,
    ExecuteCommandOptions, ExecuteCommandParams,
};

use super::LspState;

const COMMAND_START_VM: &str = "startVm";
const COMMAND_START_VM_FRIENDLY: &str = "Start alia VM";

const COMMAND_STOP_VM: &str = "stopVm";
const COMMAND_STOP_VM_FRIENDLY: &str = "Stop alia VM";

const COMMAND_EXEC_TOPLEVEL: &str = "execToplevel";
const COMMAND_EXEC_TOPLEVEL_FRIENDLY: &str = "Execute top-level form under cursor in running VM";

pub(super) fn code_action_provider() -> Option<CodeActionProviderCapability> {
    Some(true.into())
}

pub(super) fn execute_command_provider() -> Option<ExecuteCommandOptions> {
    Some(ExecuteCommandOptions {
        commands: vec![
            COMMAND_START_VM.to_string(),
            COMMAND_STOP_VM.to_string(),
            COMMAND_EXEC_TOPLEVEL.to_string(),
        ],
        ..Default::default()
    })
}

pub(super) fn list(
    params: CodeActionParams,
    ls: &mut LspState,
) -> Result<Option<CodeActionResponse>, ResponseError> {
    let mut result: CodeActionResponse = vec![];

    if !ls.vm_running {
        result.push(
            Command::new(
                COMMAND_START_VM_FRIENDLY.to_string(),
                COMMAND_START_VM.to_string(),
                None,
            )
            .into(),
        );
    } else {
        result.push(
            Command::new(
                COMMAND_EXEC_TOPLEVEL_FRIENDLY.to_string(),
                COMMAND_EXEC_TOPLEVEL.to_string(),
                Some(vec![serde_json::to_value(params.range).unwrap()]),
            )
            .into(),
        );
        result.push(
            Command::new(
                COMMAND_STOP_VM_FRIENDLY.to_string(),
                COMMAND_STOP_VM.to_string(),
                None,
            )
            .into(),
        );
    }
    Ok(Some(result))
}

pub(super) fn execute(
    params: ExecuteCommandParams,
    ls: &mut LspState,
) -> Result<bool, ResponseError> {
    if params.command == COMMAND_START_VM {
        assert!(!ls.vm_running);
        ls.vm_running = true;
    } else if params.command == COMMAND_STOP_VM {
        assert!(ls.vm_running);
        ls.vm_running = false;
    } else if params.command == COMMAND_EXEC_TOPLEVEL {
        assert!(ls.vm_running);
        assert!(params.arguments.len() == 1);
        let _range: lsp_types::Range = serde_json::from_value(params.arguments[0].clone()).unwrap();
    } else {
        panic!("unknown command {:?}", params.command);
    }
    Ok(true)
}
