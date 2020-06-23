use crate::prelude::*;

type EventContext<'a> = neon::context::TaskContext<'a>;

pub trait EventHandlerExt {
    fn emit<F>(&self, args_cb: F)
    where
        F: for<'a> FnOnce(&mut EventContext<'a>) -> Vec<Handle<'a, JsValue>>,
        F: Send + 'static;
}

impl EventHandlerExt for EventHandler {
    fn emit<F>(&self, args_cb: F)
    where
        F: for<'a> FnOnce(&mut EventContext<'a>) -> Vec<Handle<'a, JsValue>>,
        F: Send + 'static,
    {
        self.schedule(args_cb)
    }
}
