//! **fetchmod - isolate/encapsulate fetch api in a module because it is all async**  
//region description
//! ## Async world
//! With the new async/.await syntax in Rust it is now very easy to write async code.
//! It is important to be carefull to NOT write sync code after async code.
//! It can be confusing that the sync code will execute before the async code !
//! Webassembly is basically javascript and uses the executor future_to_promise()
//! The async fn that is called must return Result<JsValue, JsValue>
//! and it cannot use any references to the stack because it is executed in another timeline.
//endregion

//region: use
use crate::log1;

use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen_futures::{JsFuture};
//endregion

/// fetch in Rust with async await
/// the result must be Result<JsValue, JsValue> because I will use future_to_promise() executor
/// or it can be used in another async fn or block
/// it returns the Response as a JsValue
pub async fn async_fetch(url: String) -> Result<JsValue, JsValue> {
    //Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    let window = unwrap!(web_sys::window());
    let resp_jsvalue = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    let text_jsvalue = JsFuture::from(resp.text()?).await?;
    log1(&unwrap!(JsValue::as_string(&text_jsvalue)));
    //return
    Ok(text_jsvalue)
}
