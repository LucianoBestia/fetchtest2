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
It can be an async fn or an async block. It is not yet executed, only prepared.  
The `.await?` means the next line of code will wait for this line to finish,
but not blocking the active thread. It will work in another timeline.  
Still nothing is executed, only prepared.  

### future_to_promise

The async code is executed by an `executor`. In Wasm it is `future_to_promise()`.  
The async fn must return Result<JsValue, JsValue> because of that.  

### spawn_local

This is the recommanded executor for futures in wasm. No need for return value.  
If there is an error it will be aborted and visible in the browser console.  

## build and run

Type and run this in the project folder  
`cargo make dev`
To install cargo-make  
`cargo install --force cargo-make`
