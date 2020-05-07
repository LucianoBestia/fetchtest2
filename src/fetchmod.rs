//! **fetchmod - isolate/encapsulate fetch api in a module because it is all async**  
//region description
//! ## Async world
//! With the new async/.await syntax in Rust it is now very easy to write async code.
//! It is important to be carefull to NOT write sync code after async code.
//! It can be confusing that the sync code will execute before the async code !
//! Webassembly is basically javascript and uses the recomanded executor spawn_local() or future_to_promise().
//! Async cannot use any references to the stack because it is executed in another timeline.
//endregion

//region: use
use crate::log1;

use wasm_bindgen::{JsValue, JsCast};
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen_futures::{JsFuture};
//endregion

/// fetch in Rust with async await for executor future_to_promise()
/// the result must be Result<JsValue, JsValue>. Any error will be returned in the Result.
pub async fn async_futopr_fetch(url: String) -> Result<JsValue, JsValue> {
    //Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    let window = unwrap!(web_sys::window());
    let resp_jsvalue = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    let text_jsvalue = JsFuture::from(resp.text()?).await?;
    log1(&unwrap!(JsValue::as_string(&text_jsvalue)));
    //return the response as jsValue object
    Ok(text_jsvalue)
}

/// fetch in Rust with async await for executor spawn_local()
/// return the response as JsValue. Any error will panic.
pub async fn async_spwloc_fetch(url: String) -> JsValue {
    //Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    let window = unwrap!(web_sys::window());
    //log1("before fetch");
    let resp_jsvalue = unwrap!(JsFuture::from(window.fetch_with_request(&request)).await);
    //log1("after fetch");
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    //log1("before text()");
    let text_jsvalue = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    //log1("after text()");
    log1(&unwrap!(JsValue::as_string(&text_jsvalue)));
    //returns response as JsValue
    text_jsvalue
}
