use crate::helpers;
use crate::prelude::*;
use crate::runtime;
use crate::types::TransferFileMeta;
use serde_json::json;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::net::{TcpListener, TcpStream};

pub fn start_server(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let config = cx.argument::<JsObject>(0)?;

    let port = config.number(&mut cx, "port")?;
    let receive_files_dir = config.string(&mut cx, "receiveFilesDir")?;
    let on_start = config.callback(&mut cx, "onStart")?;
    let on_receive_file_start = config.callback(&mut cx, "onReceiveFileStart")?;
    let on_receive_file_progress = config.callback(&mut cx, "onReceiveFileProgress")?;
    let on_receive_file_complete = config.callback(&mut cx, "onReceiveFileComplete")?;
    let on_receive_file_error = config.callback(&mut cx, "onReceiveFileError")?;
    let on_server_error = config.callback(&mut cx, "onServerError")?;

    runtime::spawn(async move {
        let result = spawn_tcp_server(
            port as u16,
            Arc::new(receive_files_dir),
            on_start,
            on_receive_file_start,
            on_receive_file_progress,
            on_receive_file_complete,
            on_receive_file_error,
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
    receive_files_dir: Arc<String>,
    on_start: EventHandler,
    on_receive_file_start: EventHandler,
    on_receive_file_progress: EventHandler,
    on_receive_file_complete: EventHandler,
    on_receive_file_error: EventHandler,
) -> crate::Result<()> {
    let mut server = TcpListener::bind((IpAddr::from([0, 0, 0, 0]), port))
        .await
        .context(format!("Failed to bind the server to port: {}", port))?;

    on_start.emit(move |_| vec![]);

    while let Some(socket) = server.next().await {
        let socket = socket.context("Failed to accept new connection")?;

        let ref_id = Arc::new(helpers::gen_uuid());
        let receive_files_dir = receive_files_dir.clone();
        let on_receive_file_start = on_receive_file_start.clone();
        let on_receive_file_progress = on_receive_file_progress.clone();
        let on_receive_file_complete = on_receive_file_complete.clone();
        let on_receive_file_error = on_receive_file_error.clone();

        tokio::spawn(async move {
            let result = handle_socket(
                ref_id.clone(),
                receive_files_dir,
                socket,
                on_receive_file_start,
                on_receive_file_progress,
                on_receive_file_complete,
            )
            .await;

            if let Err(err) = result {
                on_receive_file_error
                    .emit(move |mut cx| vec![cx.string(ref_id.as_str()).upcast(), cx.string(err.to_string()).upcast()])
            }
        });
    }

    Ok(())
}

async fn handle_socket(
    ref_id: Arc<String>,
    receive_files_dir: Arc<String>,
    mut socket: TcpStream,
    on_receive_file_start: EventHandler,
    on_receive_file_progress: EventHandler,
    on_receive_file_complete: EventHandler,
) -> crate::Result<()> {
    let peer_addr = socket
        .peer_addr()
        .context("Failed to get peer remote address while receiving file")?;

    let transfer_meta = socket
        .read_json::<TransferFileMeta>()
        .await
        .context("Failed to read transfer-meta json")?;

    let cloned_ref_id = ref_id.clone();
    let from_meta = json!({
      "ip": peer_addr.ip().to_string(),
      "port": peer_addr.port()
    });
    let file_mata = json!({
      "name": transfer_meta.name.as_str(),
      "size": transfer_meta.size
    });
    on_receive_file_start.emit(move |mut cx| {
        vec![
            cx.string(cloned_ref_id.as_str()).upcast(),
            neon_serde::to_value(cx, &from_meta).unwrap(),
            neon_serde::to_value(cx, &file_mata).unwrap(),
        ]
    });

    let output_file_path =
        helpers::generate_file_path_with_available_name(receive_files_dir.as_str(), transfer_meta.name.as_str())
            .context("Failed to generate a new file path for the receiving file")?;

    let mut out_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_path.as_path())
        .await
        .context("Failed to create a output file for the receiving file")?;

    helpers::pipe(&mut socket, &mut out_file, |progress| {
        let cloned_ref_id = ref_id.clone();
        on_receive_file_progress.emit(move |mut cx| {
            vec![
                cx.string(cloned_ref_id.as_str()).upcast(),
                cx.number(progress as f64).upcast(),
            ]
        });
    })
    .await
    .context("Failed to pipe socket data to the output file")?;

    let cloned_ref_id = ref_id.clone();
    on_receive_file_complete.emit(move |mut cx| {
        vec![
            cx.string(cloned_ref_id.as_str()).upcast(),
            cx.string(output_file_path.to_str().unwrap()).upcast(),
        ]
    });

    Ok(())
}
