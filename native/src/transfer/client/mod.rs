use crate::helpers;
use crate::prelude::*;
use crate::runtime;
use crate::types::TransferFileMeta;
use serde_json::json;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::net::{lookup_host, TcpStream};

pub fn send_file(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let config = cx.argument::<JsObject>(0)?;

    let ip = config.string(&mut cx, "ip")?;
    let port = config.number(&mut cx, "port")?;
    let file_path = config.string(&mut cx, "filePath")?;
    let on_send_file_start = config.callback(&mut cx, "onSendFileStart")?;
    let on_send_file_progress = config.callback(&mut cx, "onSendFileProgress")?;
    let on_send_file_complete = config.callback(&mut cx, "onSendFileComplete")?;
    let on_send_file_error = config.callback(&mut cx, "onSendFileError")?;

    runtime::spawn(async move {
        let ref_id = Arc::new(helpers::gen_uuid());

        let result = transfer_file(
            ref_id.clone(),
            ip,
            port as u16,
            file_path,
            on_send_file_start,
            on_send_file_progress,
            on_send_file_complete,
        )
        .await;

        if let Err(err) = result {
            on_send_file_error
                .emit(move |mut cx| vec![cx.string(ref_id.as_str()).upcast(), cx.string(err.to_string()).upcast()])
        }
    });

    Ok(cx.undefined())
}

async fn transfer_file(
    ref_id: Arc<String>,
    ip: String,
    port: u16,
    file_path: String,
    on_send_file_start: EventHandler,
    on_send_file_progress: EventHandler,
    on_send_file_complete: EventHandler,
) -> crate::Result<()> {
    let mut socket = TcpStream::connect((ip.as_str(), port))
        .await
        .context(format!("Failed to connect with the recipient server: {}:{}", ip, port))?;

    let file_path = PathBuf::from(file_path)
        .canonicalize()
        .context("Selected source file does not exist")?;

    let name = file_path.file_name().and_then(|name| name.to_str()).unwrap_or("file");
    let size = file_path
        .metadata()
        .context("Failed to get metadata for the selected source file")?
        .len();

    let cloned_ref_id = ref_id.clone();
    let event_data = json!({
      "name": name,
      "size": size
    });
    on_send_file_start.emit(move |mut cx| {
        vec![
            cx.string(cloned_ref_id.as_str()).upcast(),
            neon_serde::to_value(cx, &event_data).unwrap(),
        ]
    });

    let transfer_meta = TransferFileMeta {
        name: name.to_owned(),
        size,
    };

    socket
        .write_json(&transfer_meta)
        .await
        .context("Failed to write transfer-meta JSON for the selected source file")?;

    let mut source_file = File::open(file_path.as_path())
        .await
        .context("Failed to open the selected source file")?;

    helpers::pipe(&mut source_file, &mut socket, |progress| {
        let cloned_ref_id = ref_id.clone();
        on_send_file_progress.emit(move |mut cx| {
            vec![
                cx.string(cloned_ref_id.as_str()).upcast(),
                cx.number(progress as f64).upcast(),
            ]
        });
    })
    .await
    .context("Failed to pipe selected source file data to socket")?;

    let cloned_ref_id = ref_id.clone();
    on_send_file_complete.emit(move |mut cx| vec![cx.string(cloned_ref_id.as_str()).upcast()]);

    Ok(())
}
