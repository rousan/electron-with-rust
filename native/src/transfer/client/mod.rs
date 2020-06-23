use crate::prelude::*;
use crate::runtime;
use std::net::SocketAddr;
use tokio::net::lookup_host;

pub fn send_file(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let config = cx.argument::<JsObject>(0)?;

    let ip = config.string(&mut cx, "ip")?;
    let port = config.number(&mut cx, "port")?;
    let file_path = config.string(&mut cx, "filePath")?;
    let on_file_send_start = config.callback(&mut cx, "onFileSendStart")?;
    let on_file_send_progress = config.callback(&mut cx, "onFileSendProgress")?;
    let on_file_send_complete = config.callback(&mut cx, "onFileSendComplete")?;
    let on_file_send_error = config.callback(&mut cx, "onFileSendError")?;

    runtime::spawn(async move {
        let result = transfer_file(
            ip,
            port as u16,
            file_path,
            on_file_send_start,
            on_file_send_progress,
            on_file_send_complete,
        )
        .await;

        if let Err(err) = result {
            on_file_send_error.emit(move |mut cx| vec![cx.string(err.to_string()).upcast()])
        }
    });

    Ok(cx.undefined())
}

async fn transfer_file(
    ip: String,
    port: u16,
    file_path: String,
    on_file_send_start: EventHandler,
    on_file_send_progress: EventHandler,
    on_file_send_complete: EventHandler,
) -> crate::Result<()> {
    todo!()
}
