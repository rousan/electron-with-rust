use crate::prelude::*;
use crate::runtime;
use tokio::time;

pub fn foo_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let cb = cx.argument::<JsFunction>(0)?;
    // let path = cx.argument::<JsString>(1)?.value();

    let this = cx.undefined();
    let event_handler = EventHandler::new(&cx, this, cb);

    runtime::spawn(async move {
        // time::delay_for(time::Duration::from_secs(10)).await;

        // let content = tokio::fs::read_to_string(path).await.unwrap();
        let users = reqwest::get("https://staging.api.charts.com/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        event_handler.schedule(move |cx| {
            let args: Vec<Handle<JsValue>> = vec![cx.string(users).upcast()];
            args
        });
    });

    Ok(cx.undefined())
}
