use crate::helpers;
use crate::prelude::*;
use serde_json::json;
use std::path::PathBuf;

pub fn gen_ref_id(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(helpers::gen_uuid()))
}

pub fn get_file_meta(mut cx: FunctionContext) -> JsResult<JsObject> {
    let file_path = PathBuf::from(cx.argument::<JsString>(0)?.value());

    let meta_data = std::fs::metadata(file_path.as_path())
        .context("Failed to get file metadata")
        .throw(&mut cx)?;

    let file_name = file_path.file_name().and_then(|name| name.to_str()).unwrap_or("");

    let meta_obj = neon_serde::to_value(
        &mut cx,
        &json!({
          "name": file_name,
          "size": meta_data.len()
        }),
    )
    .wrap()
    .throw(&mut cx)?
    .downcast_or_throw::<JsObject, _>(&mut cx)?;

    Ok(meta_obj)
}
