# fetchtest2

*Things are changing fast. This is the situation on 2020-01-05.*

I am trying to use the fetch api in Rust Wasm.  
It is such a frequent functionality for frontend applications.  
Rust has now async/await. This is great for interaction with javascript async/await.  
My first experiments with my project "fetchtest" are now obsolete. Forget it.  

## js async + await

Javascript versions have async+await. That looks really good readable code. Finally.  
<https://fundamentalsofcode.com/javascript-fetch-api-and-using-async-await/>  

## rust async + await

The async world is like having more timelines with code and data (stack).  
In one time usually is only one timeline that advances and other are waiting.  
The `async` word means that this code will be prepared in another timeline.  
It can be an `async fn` or an `async block`. It is not yet executed, only prepared.  
The `.await?` means the next line of code will wait for this line to finish,
but not blocking the active thread. It will work in "another timeline" eventually.  
Still nothing is executed, only prepared.  

### future_to_promise()

The async code is executed by an `executor`.  
The javascript executor in Wasm is `wasm_bindgen_futures::future_to_promise()`.  
The async fn must return `Result<JsValue, JsValue>` because of that.  
But there is no need to use the javascript executor anymore.  

### spawn_local()

`wasm_bindgen_futures::spawn_local()` is the recommanded executor for futures in wasm.  
No need for return value or javascript.  
If there is an error it will be aborted and visible in the browser console.  

## dodrio vdom

Dodrio vdom contains the struct RootRenderingComponent rrc.  
All the data are there and from there are usually rendered to the screen.  
I cannot send the reference of rrc to my async functions. They must have only 'static parameters.  
But I can send a clone of VdomWeak, because it is omnipresent.  
To change the values inside the `rrc` I must use the `with_components()` future.  
The version of `dodrio` is `0.1.0` in `crate.io` and this is too old for the new futures library.  
I hope that the Git master version will have this corrected.  
The `with_components()` will change the data on the next tick to avoid data races.  

## build and run

Type and run this in the project folder  
`cargo make dev` or `cargo make release`  
To install cargo-make  
`cargo install --force cargo-make`  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://bestia.dev/cargo_crev_web/query/num-traits>  

