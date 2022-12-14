# Theory

WebAssembly (WASM) is an assembly format designed to be portable and fast to execute. It started as an alternative to JavaScript for the web, but now it can be used on non-web platforms as well. The advantages of WASM include:
- can run in the browser;
- many languages (Rust, C, C++, C#, etc.) can be compiled into WASM;
- can be easily translated into ASM understood by hardware with low performance penalty compared to the native version;
- it's fully sandboxed from other processes/threads/foreign code (can't damage the system in any way);

For example, this code:
```rs
pub fn f(x: u32, y: u32, z: u32) -> u32 {
    x + y * z
}
```
Compiles to:
```wasm
example::f:
        ; local.get = get the nth local variable
        local.get       2 ; z
        local.get       1 ; y
        i32.mul           ; z * y
        local.get       0 ; x
        i32.add           ; the multiplication result + x
                          ; implicit return of the last value
        end_function
```

The main disadvantage of WASM at this moment is the fact that it can't manipulate the browser's Document Object Model (DOM); that is, the graph with all the UI elements and styles that are shown to the user.

Because of this, and the fact that WASM doesn't have access to any external code by default, it means that we still need some JavaScript (JS) to glue the Rust code to the UI elements of the browser.

[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) is used to automate some tasks done when creating WASM files for the web.

[wasm-bindgen](https://crates.io/crates/wasm-bindgen) is used to generate the glue code between Rust and JS (example from the page):
```rs
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

To return errors from the function, the error type must be either `JsValue`, or convertible to `JsValue` (ignore the `pub` keyword for now):
```rs
#[wasm_bindgen]
pub fn try_get_string(x: bool) -> Result<String, JsValue> {
    if x {
        Ok("abc".to_string())
    } else {
        Err(JsValue::from("some error"))
    }
}
```

Because JS' way of doing errors is though exceptions, if the `Result` happens to be an error, a JS exception is thrown on the JS side; otherwise, the success value is unwraped and returned:
```js
try {
    console.log(try_get_string(true)); // works
    console.log(try_get_string(false)); // exception
} catch (ex) {
    console.log(ex); // prints the exception string
}
```

An example crate is available at [fiirust_wasm](./fiirust_wasm):
- `src/lib.rs` contains the Rust code for the project;
- `index.html` is the markup code for some simple UI;
- `index.js` is the JS code that hooks into the DOM events and executes Rust code when they happen (eg. a button is clicked);

To build the project for the web, run the below command in a terminal. Now, in the `pkg` folder, you should have the `.wasm` file with the binary code, and some JS code that knows how to call the exported functions.
```
wasm-pack build --target=web
```

Now we need a web server to serve the code to a browser. We'll use the one that comes by default with Python. Run the below command in the `fiirust_wasm` folder (not `pkg`!). You can leave this terminal opened; this doesn't need to be reloaded.
```
python -m http.server
```

Now you should be able to access the [site](http://localhost:8000) from the browser.

# Problems

## P1

The `base64_encode` function should convert the input buffer into a [base64](https://en.wikipedia.org/wiki/Base64) encoded string. The only modifications you should do are in the `src/lib.rs` file.

Remember to run the build command after each code change, and to refresh the page in the browser with `Ctrl+F5`. This is necessary because the browser will cache the page without it and not update it to the latest version.

## P2

Implement the algorithm for decoding base64 (hint: what are the input and output types?) and modify the HTML and the JS to make it usable from the web.