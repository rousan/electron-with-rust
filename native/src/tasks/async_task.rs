use crate::prelude::*;
// use libuv::Loop;
use std::ops::Deref;
use std::thread;

struct FetchTask;

impl Task for FetchTask {
    type Output = ();
    type Error = crate::Error;
    type JsEvent = JsUndefined;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        println!("libiv task: {:?}", thread::current().id());
        Ok(())
    }

    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        println!("libiv complete: {:?}", thread::current().id());
        Ok(cx.undefined())
    }
}

// struct SendifyOwned {
//     val: *const JsObject,
// }
//
// unsafe impl Send for SendifyOwned {}
// unsafe impl Sync for SendifyOwned {}

pub fn async_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let undefined = cx.undefined();

    // let cb = cx.argument::<JsFunction>(0).unwrap();
    // FetchTask.schedule(cb);
    let cb = JsFunction::new(&mut cx, |mut cx| {
        println!("i am from main thread loop: {:?}", thread::current().id());
        Ok(cx.undefined())
    })
    .unwrap();
    let this = cx.this();
    let eh = EventHandler::new(&cx, this, cb);

    thread::spawn(move || {
        println!("sleeping: {:?}", thread::current().id());
        // thread::sleep(std::time::Duration::from_secs(5));
        println!("sleeping dones");
        eh.schedule(move |cx| {
            println!("inside schedule {:?}", thread::current().id());
            let args: Vec<Handle<JsValue>> = vec![cx.string("abc").upcast(), cx.number(100).upcast()];
            args
        });
        println!("scheduled");
    });

    Ok(undefined)
}
