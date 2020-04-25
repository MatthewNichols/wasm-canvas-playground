# wasm-canvas-playground
A space to explore programming HTML Canvas drawing with most of the coding happening in Rust/WebAssembly. There will likely be a bent toward physics and game oriented demos.

## Why Rust?
I recently took up Rust because I am VERY excited about the possibilities that WebAssembly raise (both in and out of the browser) and at the moment Rust is one of the few languages that can compile to WebAssembly. I am using this as a place to practice Rust coding because graphics are a lot more fun than text and I know HTML Canvas.  

## Setup
The project is set up from the [rust-webpack](https://github.com/rustwasm/rust-webpack-template) template which allows writing Rust code in the same project as the web code. This is compiled into a webAssembly by webpack and auto updates in your browser just like any other webpack project. 
In lib.rs there is a function called main_js that will be called when the webpage is loaded. Once that is running rust code is in charge. The drawing mechanics will be done by calling some custom js functions that will be injected into the WASM when it is invoked. The amount of js code will be kept to a minimum.

## Installation

I am developing on Windows 10 under Windows Subsystem for Linux 2. I suspect it will work pretty portably, but let me know in issues if not.

- Install Node & npm
- Please see the RustWasm docs for prerequisites (https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html)
- Clone this repos
- Run ```npm install``` to set up
- Run ```npm start``` to run in debug mode

See the copy of the base rust-webpack ReadMe [here](rust-webpack-README.md) for more details.

## Development Details

### Step 1: Bridging to the browser
There is a js part to this (js/canvas-writer.js) and a rust part (js_bridge.rs). The js side is a module with any exported functions I want to be able to call from Rust. Then in js_bridge.rs I use

```rust
#[wasm_bindgen(module = "/js/canvas-writer.js")]
extern {
    pub fn init(canvas_id: &str, height: i32, width: i32);
    pub fn clear(color_code: &str);
    #[wasm_bindgen(js_name=drawCircle)]
    pub fn draw_circle(center_x: i32, center_y: i32, radius: i32, color_r: u8, color_g: u8, color_b: u8, color_a: f32);
}
```
to map the js functions into Rust. Note the ```#[wasm_bindgen(js_name=drawCircle)]``` line that let's me rename the method called in Rust to snake case the way that the Rust community (Rusticans?) prefers.

Then in lib.rs I reference the bridge code with 

```rust
mod js_bridge;
```

and in main_js() i added 

```rust
js_bridge::init("canvas1", 1000, 1000);
js_bridge::draw_circle(500, 500, 200, 255, 128, 128, 1.0);
```
to draw a quick circle and demo that it works. In future drawing code will get pulled out to separate files.

If you run ```npm start``` and open a a browser to http://localhost:8080/ you should see a colored circle.