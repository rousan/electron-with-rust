use crate::prelude::*;
use neon::object::PropertyKey;

pub trait JSObjectExt {
    fn prop<'a, C: Context<'a>, K: PropertyKey, T: Value>(self, cx: &mut C, key: K) -> NeonResult<Handle<'a, T>>;
    fn number<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<f64>;
    fn string<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<String>;
    fn func<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<Handle<'a, JsFunction>>;
    fn callback<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<EventHandler>;
}

impl<O: Object> JSObjectExt for O {
    fn prop<'a, C: Context<'a>, K: PropertyKey, T: Value>(self, cx: &mut C, key: K) -> NeonResult<Handle<'a, T>> {
        self.get(cx, key)?.downcast_or_throw(cx)
    }

    fn number<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<f64> {
        let num: Handle<JsNumber> = self.prop(cx, key)?;
        Ok(num.value())
    }

    fn string<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<String> {
        let string: Handle<JsString> = self.prop(cx, key)?;
        Ok(string.value())
    }

    fn func<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<Handle<'a, JsFunction>> {
        self.prop(cx, key)
    }

    fn callback<'a, C: Context<'a>, K: PropertyKey>(self, cx: &mut C, key: K) -> NeonResult<EventHandler> {
        let cb = self.func(cx, key)?;
        let this = cx.undefined();
        Ok(EventHandler::new(cx, this, cb))
    }
}
