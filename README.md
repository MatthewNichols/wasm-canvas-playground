# wasm-canvas-playground
A space to explore programming HTML Canvas drawing with most of the coding happening in Rust/WebAssembly. There will likely be a bent toward physics and game oriented demos.

## Why Rust?
I recently took up Rust because I am VERY excited about the possibilities that WebAssembly raise (both in and out of the browser) and at the moment Rust is one of the few languages that can compile to WebAssembly. I am using this as a place to practice Rust coding because graphics are a lot more fun than text and I know HTML Canvas.  

## Approach


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


