use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

struct FetchTask {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RespData {
    code: u128,
    data: Vec<Row>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    #[serde(rename = "Province/State")]
    state: String,
    #[serde(rename = "Country/Region")]
    country: String,
    #[serde(rename = "Lat")]
    lat: f64,
    #[serde(rename = "Long")]
    long: f64,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Confirmed")]
    confirmed: u64,
    #[serde(rename = "Deaths")]
    deaths: u64,
    #[serde(rename = "Recovered")]
    recovered: u64,
    #[serde(rename = "WHO Region")]
    who: String,
}

impl Task for FetchTask {
    type Output = RespData;
    type Error = crate::Error;
    type JsEvent = JsValue;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        // let resp = reqwest::blocking::get(&self.url).context("Couldn't fetch url")?;
        //
        // let code = resp.status().as_u16();
        // let data = resp.json::<Value>().context("Failed to convert response to json")?;

        let t = std::time::Instant::now();
        let data = fs::read_to_string("/Users/rousan/Downloads/cars.json").wrap()?;
        let data = serde_json::from_str(&data).wrap()?;
        let t = t.elapsed().as_millis();

        Ok(RespData { code: t, data })
    }

    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        let resp_data = result.throw(&mut cx)?;
        let t = std::time::Instant::now();
        let f = neon_serde::to_value(&mut cx, &resp_data).throw(&mut cx);
        let t = t.elapsed().as_millis();
        println!("rust in: {}", t);
        f
    }
}

pub fn fetch_url(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let url = cx.argument::<JsString>(0)?;
    let cb = cx.argument::<JsFunction>(1)?;

    let task = FetchTask { url: url.value() };
    task.schedule(cb);

    Ok(cx.undefined())
}
