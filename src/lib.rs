//! **fetchtest2**  

//needed for dodrio! macro (typed-html)
#![recursion_limit = "512"]
//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
//endregion

//region: extern and use statements
mod fetchmod;

extern crate console_error_panic_hook;
extern crate log;
extern crate serde;
extern crate strum;
extern crate strum_macros;
extern crate web_sys;
#[macro_use]
extern crate unwrap;
extern crate conv;

use dodrio::builder::text;
use wasm_bindgen::prelude::*;
use dodrio::bumpalo::{self};
use dodrio::{Node, Render, RenderContext};
use typed_html::dodrio;
use serde::{Deserialize, Serialize};
use web_sys::{console};
use wasm_bindgen_futures::{future_to_promise, spawn_local};
//endregion

///simple console write with a string
fn log1(x: &str) {
    console::log_1(&JsValue::from_str(x));
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRenderingComponent {
    pub respbody: String,
}

#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(
        document.get_element_by_id("div_for_virtual_dom"),
        "No #div_for_virtual_dom"
    );

    // Construct a new rendering component.
    let rrc = RootRenderingComponent::new();

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Run the component forever.
    vdom.forget();
}

impl RootRenderingComponent {
    fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            respbody: "".to_owned(),
        }
    }
}

// The `Render` implementation. It is called for every Dodrio animation frame to render the vdom.
impl Render for RootRenderingComponent {
    #[allow(clippy::panic)]
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        //create the virtual dom
        let bump = cx.bump;
        dodrio!(bump,
            <div>
                <h1>
                    {vec![text(
                        bumpalo::format!(in bump, "rust future_to_promise{}",
                        "")
                        .into_bump_str()
                    )]}
                </h1>
                <button style= "margin:auto;display:block;" onclick={ move |root, vdom, _event| {
                    //future_to_promise() is an executor that works with wasm.
                    let url ="https://jsonplaceholder.typicode.com/todos/1".to_owned();
                    future_to_promise(async_futopr_fetch_and_write(url));
                    //The code after the async executor will not wait for the async execution.
                    //In most cases having code here is misplaced.
                    log1("end of on futopr click");
                    }}>"fetch rust async await fu. to pr."
                </button>
                <div id="for_fetch_rust_futopr_async_await">
                </div>
                <h1>
                    {vec![text(
                        bumpalo::format!(in bump, "rust spawn_local{}",
                        "")
                        .into_bump_str()
                    )]}
                </h1>
                <button style= "margin:auto;display:block;" onclick={ move |root, vdom, _event| {
                    let v2=vdom.clone();
                    //async executor spawn_local is the recommanded for wasm
                    let url ="https://jsonplaceholder.typicode.com/todos/1".to_owned();
                    spawn_local(async_spwloc_fetch_and_write(url));
                    //The code after the async executor will not wait for the async execution.
                    //In most cases having code here is misplaced.
                    log1("end of on spwloc click");
                    }}>"fetch rust async await spawn_local."
                </button>
                <div id="for_fetch_rust_spwloc_async_await">
                </div>
                <h1>
                    {vec![text(
                    bumpalo::format!(in bump, "spawn_local with update rrc.respbody{}",
                    "")
                    .into_bump_str()
                    )]}
                </h1>
                <button style= "margin:auto;display:block;" onclick={ move |root, vdom, _event| {
                    let v2=vdom.clone();
                    //async executor spawn_local is the recommanded for wasm
                    let url ="https://jsonplaceholder.typicode.com/todos/1".to_owned();
                    spawn_local(async_spwloc_fetch_and_rrcwrite(url,v2));
                    //The code after the async executor will not wait for the async execution.
                    //In most cases having code here is misplaced.
                    log1("end of on spwloc rrc_click");
                    }}>"fetch spawn_local with update rrc"
                </button>
                <div >
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        self.respbody)
                        .into_bump_str()
                    )]}
                </div>
            </div>
        )
    }
}

//region: fetch in Rust with async await

/// not recomanded
/// the async fn for executor futopr == future_to_promise
/// the result must be Result<JsValue, JsValue>
/// no references to stack allowed beause it is executed in another timeline unknown when
pub async fn async_futopr_fetch_and_write(url: String) -> Result<JsValue, JsValue> {
    let text_jsvalue = fetchmod::async_futopr_fetch(url).await?;
    write_result(&text_jsvalue, "for_fetch_rust_futopr_async_await");
    log1("end of async_futopr_fetch_and_write()");
    Ok(text_jsvalue)
}

/// recomanded: the async fn for executor spwloc == spawn_local
/// the result must be ()
/// no references to stack allowed beause it is executed in another timeline unknown when
pub async fn async_spwloc_fetch_and_write(url: String) {
    let text_jsvalue = fetchmod::async_spwloc_fetch(url).await;
    write_result(&text_jsvalue, "for_fetch_rust_spwloc_async_await");
    log1("end of async_spwloc_fetch_and_write()");
}

/// recomanded: the async fn for executor spwloc == spawn_local
/// with update the value in struct rrc with await
pub async fn async_spwloc_fetch_and_rrcwrite(url: String, vdom: dodrio::VdomWeak) {
    let text_jsvalue = fetchmod::async_spwloc_fetch(url).await;
    let txt_str: String = unwrap!(JsValue::as_string(&text_jsvalue));
    //update values in rrc is async
    //I can use a fn call or an async block
    //update_rrc(vdom, txt_str).await;

    async {
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    rrc.respbody = txt_str;
                }
            })
            .await
        );
        vdom.schedule_render();
    }
    .await;

    log1("end of async_spwloc_fetch_and_rrcwrite()");
}

///update rrc
///the vdom is omnipresent. To change a value in the rrc struct I must use the Vdom.
///the with_components will execute on the next browser tick to avoid data races
pub async fn update_rrc(vdom: dodrio::VdomWeak, txt_str: String) {
    unwrap!(
        vdom.with_component({
            move |root| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                rrc.respbody = txt_str;
            }
        })
        .await
    );
    vdom.schedule_render();
}

/// this is just a normal sync function
/// it can be called from an async fn too.
/// this is a raw writing to the DOM - not the virtual DOM
/// it is o for temporary debugging, but not of a permanent value
fn write_result(text_jsvalue: &JsValue, div_id: &str) {
    let txt: String = unwrap!(JsValue::as_string(text_jsvalue));
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_fetch_rust_futopr_async_await = unwrap!(document.get_element_by_id(div_id));
    div_for_fetch_rust_futopr_async_await.set_inner_html(&txt);
}

//endregion
