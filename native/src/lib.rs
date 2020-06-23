use neon::register_module;

pub use error::Error;

mod error;
mod prelude;
mod runtime;
mod tasks;

register_module!(mut cx, {
    cx.export_function("nativeFetchUrl", tasks::fetch_url).unwrap();
    cx.export_function("nativeAsyncTask", tasks::async_task).unwrap();
    cx.export_function("nativeStartRuntime", runtime::start_runtime)
        .unwrap();
    cx.export_function("nativeShutdownRuntime", runtime::shutdown_runtime)
        .unwrap();
    cx.export_function("nativeFooTask", tasks::foo_task).unwrap();
    Ok(())
});
