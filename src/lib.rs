mod utils;

use wasm_bindgen::prelude::*;
use web_sys::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
// #[wasm_bindgen]
// pub fn greet() {
//     using_web_sys();
//     alert("Hello, {{project-name}}!");
// }
// And finally, we don't even have to define the `log` function ourselves! The
// `web_sys` crate already has it defined for us.

// fn using_web_sys() {
//     console::log_1(&"Hello using web-sys".into());

//     let js: JsValue = 4.into();
//     console::log_2(&"Logging arbitrary values looks like".into(), &js);
// }
// Called when the wasm module is instantiated
// #[wasm_bindgen(start)]
// pub fn main() -> Result<(), JsValue> {
//     // Use `web_sys`'s global `window` function to get a handle on the global
//     // window object.
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     // Manufacture the element we're gonna append
//     let val = document.create_element("p")?;
//     val.set_inner_html("Hello from Rust!");

//     body.append_child(&val)?;

//     Ok(())
// }
#[wasm_bindgen]
pub fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut a = 0;
        let mut b = 1;
        for _i in 2..n {
            let c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}
