use crate::prelude::*;
use crate::runtime;
use tokio::net::TcpListener;

pub fn start_server(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let config = cx.argument::<JsObject>(0)?;

    let port = config.number(&mut cx, "port")?;
    let on_start = config.callback(&mut cx, "onStart")?;
    let on_file_receive_start = config.callback(&mut cx, "onFileReceiveStart")?;
    let on_file_receive_progress = config.callback(&mut cx, "onFileReceiveProgress")?;
    let on_file_receive_complete = config.callback(&mut cx, "onFileReceiveComplete")?;
    let on_file_receive_error = config.callback(&mut cx, "onFileReceiveError")?;
    let on_server_error = config.callback(&mut cx, "onServerError")?;

    runtime::spawn(async move {
        let result = spawn_tcp_server(
            port as u16,
            on_start,
            on_file_receive_start,
            on_file_receive_progress,
            on_file_receive_complete,
            on_file_receive_error,
        )
        .await;

        if let Err(err) = result {
            // It means server is shutdown for some reason.
            on_server_error.emit(move |mut cx| vec![cx.string(err.to_string()).upcast()])
        }
    });

    Ok(cx.undefined())
}

async fn spawn_tcp_server(
    port: u16,
    on_start: EventHandler,
    on_file_receive_start: EventHandler,
    on_file_receive_progress: EventHandler,
    on_file_receive_complete: EventHandler,
    on_file_receive_error: EventHandler,
) -> crate::Result<()> {
    Err(crate::Error::new("fsfsdf"))
}
