use std::error::Error;
use std::fmt::Write;

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response, ResponseError};
use lsp_textdocument::TextDocuments;
use lsp_types::request::{CodeActionRequest, ExecuteCommand, GotoDefinition, HoverRequest};
use lsp_types::{
    CodeActionResponse, Command, ExecuteCommandOptions, GotoDefinitionResponse, Hover,
    HoverContents, InitializeParams, MarkupContent, MarkupKind, OneOf, Position, Range,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

use crate::parser::Document;

const COMMAND_START_VM: &str = "startVm";
const COMMAND_START_VM_FRIENDLY: &str = "Start alia VM";

const COMMAND_STOP_VM: &str = "stopVm";
const COMMAND_STOP_VM_FRIENDLY: &str = "Stop alia VM";

const COMMAND_EXEC_TOPLEVEL: &str = "execToplevel";
const COMMAND_EXEC_TOPLEVEL_FRIENDLY: &str = "Execute top-level form under cursor in running VM";

pub(crate) fn main(args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    eprintln!("alia lsp server starting");

    if args.len() != 0 {
        return Err("lsp doesn't take any args".into());
    }

    let (connection, io_threads) = Connection::stdio();
    let mut documents = TextDocuments::new();
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        hover_provider: Some(true.into()),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        document_formatting_provider: Some(OneOf::Left(true)),
        definition_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(true.into()),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec![COMMAND_START_VM.to_string()],
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    main_loop(connection, initialization_params, &mut documents)?;
    io_threads.join()?;

    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    documents: &mut TextDocuments,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut vm_running = false;

    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                let req = match cast::<HoverRequest>(req) {
                    Ok((id, params)) => {
                        let pos = params.text_document_position_params.position;
                        let uri = &params.text_document_position_params.text_document.uri;
                        let content = documents.get_document_content(uri, None).unwrap();
                        if let Ok(doc) = content.parse::<Document>() {
                            let mut value = String::new();
                            writeln!(value, "# {}", "thing")?;
                            writeln!(value, "Some text")?;
                            writeln!(value, "```lisp")?;
                            writeln!(value, "(um yea ['hi])")?;
                            writeln!(value, "{}", doc)?;
                            writeln!(value, "```")?;

                            let result = Some(Hover {
                                contents: HoverContents::Markup(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value,
                                }),
                                range: Some(Range::new(
                                    pos,
                                    Position::new(pos.line, pos.character + 3),
                                )), // XXX
                            });
                            connection.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(&result).unwrap()),
                                error: None,
                            }))?;
                        } else {
                            connection.sender.send(Message::Response(Response {
                                id,
                                result: Some(serde_json::to_value(&None::<Hover>).unwrap()),
                                error: None,
                            }))?;
                        }
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                let req = match cast::<GotoDefinition>(req) {
                    Ok((id, _params)) => {
                        let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                        connection.sender.send(Message::Response(Response {
                            id,
                            result: Some(serde_json::to_value(&result).unwrap()),
                            error: None,
                        }))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                let req = match cast::<CodeActionRequest>(req) {
                    Ok((id, params)) => {
                        let mut result: CodeActionResponse = vec![];

                        if !vm_running {
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
                        let result = Some(result);
                        connection.sender.send(Message::Response(Response {
                            id,
                            result: Some(serde_json::to_value(&result).unwrap()),
                            error: None,
                        }))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                let req = match cast::<ExecuteCommand>(req) {
                    Ok((id, params)) => {
                        if params.command == COMMAND_START_VM {
                            assert!(!vm_running);
                            vm_running = true;
                        } else if params.command == COMMAND_STOP_VM {
                            assert!(vm_running);
                            vm_running = false;
                        } else if params.command == COMMAND_EXEC_TOPLEVEL {
                            assert!(vm_running);
                            assert!(params.arguments.len() == 1);
                            let _range: Range =
                                serde_json::from_value(params.arguments[0].clone()).unwrap();
                        } else {
                            panic!("unknown command {:?}", params.command);
                        };
                        connection.sender.send(Message::Response(Response {
                            id,
                            result: Some(serde_json::to_value(true).unwrap()),
                            error: None,
                        }))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                connection.sender.send(Message::Response(Response {
                    id: req.id,
                    result: None,
                    error: Some(ResponseError {
                        code: 1, // XXX
                        message: "weh".to_string(),
                        data: None,
                    }),
                }))?;
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                if !documents.listen(not.method.as_str(), &not.params) {
                    eprintln!("got notification: {not:?}");
                }
            }
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
