use std::error::Error;

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response, ResponseError};
use lsp_textdocument::TextDocuments;
use lsp_types::request::{CodeActionRequest, ExecuteCommand, GotoDefinition, HoverRequest};
use lsp_types::{
    InitializeParams, OneOf, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

mod action;
mod goto;
mod hover;

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
        code_action_provider: action::code_action_provider(),
        execute_command_provider: action::execute_command_provider(),
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

macro_rules! lsp_handler {
    ($req:ident => $ty:ty, $handler:path[$ls: ident]) => {
        let $req = match cast::<$ty>($req) {
            Ok((id, params)) => {
                let response = match $handler(params, &mut $ls) {
                    Ok(value) => Response {
                        id,
                        result: Some(serde_json::to_value(&value).unwrap()),
                        error: None,
                    },
                    Err(err) => Response {
                        id,
                        result: None,
                        error: Some(err),
                    },
                };
                $ls.connection.sender.send(Message::Response(response))?;
                continue;
            }
            Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
            Err(ExtractError::MethodMismatch(req)) => req,
        };
    };
}

struct LspState<'c, 'd> {
    connection: &'c Connection,
    documents: &'d mut TextDocuments,
    vm_running: bool,
}

impl<'c, 'd> LspState<'c, 'd> {
    fn new(connection: &'c Connection, documents: &'d mut TextDocuments) -> LspState<'c, 'd> {
        LspState {
            connection,
            documents,
            vm_running: false,
        }
    }
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    documents: &mut TextDocuments,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut ls = LspState::new(&connection, documents);

    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                lsp_handler!(req => HoverRequest, hover::handle[ls]);
                lsp_handler!(req => GotoDefinition, goto::handle[ls]);
                lsp_handler!(req => CodeActionRequest, action::list[ls]);
                lsp_handler!(req => ExecuteCommand, action::execute[ls]);
                connection.sender.send(Message::Response(Response {
                    id: req.id,
                    result: None,
                    error: Some(ResponseError {
                        code: 1, // XXX
                        message: "unhandled command".to_string(),
                        data: None,
                    }),
                }))?;
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                if !ls.documents.listen(not.method.as_str(), &not.params) {
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
