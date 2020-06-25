use neon::register_module;

pub use error::Error;

mod error;
mod ext;
mod helpers;
mod prelude;
mod runtime;
mod transfer;
mod types;
mod utils;

pub type Result<T> = std::result::Result<T, crate::Error>;

register_module!(mut cx, {
    cx.export_function("nativeStartTokioRuntime", runtime::start_runtime)
        .unwrap();
    cx.export_function("nativeShutdownTokioRuntime", runtime::shutdown_runtime)
        .unwrap();
    cx.export_function("nativeStartServer", transfer::start_server).unwrap();
    cx.export_function("nativeSendFile", transfer::send_file).unwrap();
    cx.export_function("nativeGenRefId", utils::gen_ref_id).unwrap();
    cx.export_function("nativeGetFileMeta", utils::get_file_meta).unwrap();

    Ok(())
});
