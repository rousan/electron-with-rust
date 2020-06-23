use crate::prelude::*;
use std::future::Future;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

static mut RUNTIME: Option<Runtime> = None;

pub fn start_runtime(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    unsafe {
        if RUNTIME.is_some() {
            return Ok(cx.undefined());
        }

        RUNTIME.replace(
            tokio::runtime::Builder::new()
                .threaded_scheduler()
                .core_threads(runtime_core_threads_count())
                .on_thread_start(|| {
                    println!("Tokio worker thread started: {:?}", std::thread::current().id());
                })
                .on_thread_stop(|| {
                    println!("Tokio worker thread stopped: {:?}", std::thread::current().id());
                })
                .enable_io()
                .enable_time()
                .build()
                .unwrap(),
        );
    }

    Ok(cx.undefined())
}

pub fn shutdown_runtime(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    if let Some(runtime) = unsafe { RUNTIME.take() } {
        println!("Shutdown tokio runtime started");
        runtime.shutdown_timeout(tokio::time::Duration::from_secs(1));
        println!("Shutdown tokio runtime done");
    }

    Ok(cx.undefined())
}

pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    unsafe { RUNTIME.as_ref().unwrap().spawn(task) }
}

fn runtime_core_threads_count() -> usize {
    num_cpus::get().min(4).max(2)
}
