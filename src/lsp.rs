use std::error::Error;

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response, ResponseError};
use lsp_types::request::{CodeActionRequest, GotoDefinition};
use lsp_types::{
    CodeActionResponse, Command, ExecuteCommandOptions, GotoDefinitionResponse, InitializeParams,
    OneOf, ServerCapabilities,
};

pub(crate) fn main(args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    eprintln!("alia lsp server starting");

    if args.len() != 0 {
        return Err("lsp doesn't take any args".into());
    }

    let (connection, io_threads) = Connection::stdio();
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(true.into()),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec!["command".to_string()],
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
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    eprintln!("baibai");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                let req = match cast::<GotoDefinition>(req) {
                    Ok((id, _params)) => {
                        let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                        let result = serde_json::to_value(&result).unwrap();
                        connection.sender.send(Message::Response(Response {
                            id,
                            result: Some(result),
                            error: None,
                        }))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                let req = match cast::<CodeActionRequest>(req) {
                    Ok((id, _params)) => {
                        let result: Option<CodeActionResponse> = Some(vec![Command::new(
                            "title".to_string(),
                            "command".to_string(),
                            None,
                        )
                        .into()]);
                        let result = serde_json::to_value(&result).unwrap();
                        connection.sender.send(Message::Response(Response {
                            id,
                            result: Some(result),
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
                eprintln!("got notification: {not:?}");
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
