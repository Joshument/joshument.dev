use yew::prelude::*;
use js_sys::Date;
use wasm_bindgen::JsValue;

#[derive(Copy, Clone, PartialEq)]
pub enum FormatOptions {
    HourMinute
}


pub fn get_local_time(unix_timestamp: u64, format_option: FormatOptions) -> String {
    let unix_timestamp = (unix_timestamp * 1000) as f64;

    let date = Date::new(&unix_timestamp.into());
    let offset = date.get_timezone_offset();

    let timezone_date = Date::new(&(unix_timestamp + offset).into());

    match format_option {
        FormatOptions::HourMinute => {
            format!(
                "{:02}:{:02}",
                timezone_date.get_hours(),
                timezone_date.get_minutes()
            )
        }
    }
}